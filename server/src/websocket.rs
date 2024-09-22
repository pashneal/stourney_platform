// allows Websocket::split()
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::{error, info, trace, warn, debug};
use splendor_arena::{models::*, SmallClientInfo};
use std::collections::HashMap;
use tokio::time::timeout;
use uuid::Uuid;
use warp::Filter;

use sqlx::sqlite::SqlitePool;
use crate::api;
use crate::queue::*;
use crate::database;
use crate::constants::HOST_NAME;

type GameLedger = Vec<SmallClientInfo>;
// May need a lock-free data structure here
type Games = HashMap<Uuid, GameLedger>;
// Ditto
pub type Arenas = HashMap<Uuid, ArenaState>;

pub type AsyncGames = std::sync::Arc<std::sync::Mutex<Games>>;
pub type AsyncArenas = std::sync::Arc<std::sync::Mutex<Arenas>>;

#[derive(Debug, Clone)]
pub struct ArenaState {
    pub authenticated: bool,           // Did the client authenticate?
    pub initialized: bool,             // Did the client initialize the game?
    pub id: Uuid,                      // The client's id set on websocket connect
    pub num_successful_updates: usize, // Number of successful updates since init
}

pub async fn serve(port: u16, db_pool: sqlx::SqlitePool) {
    let arenas = std::sync::Arc::new(std::sync::Mutex::new(Arenas::new()));
    let games = std::sync::Arc::new(std::sync::Mutex::new(Games::new()));

    let db_clone = db_pool.clone();


    let arenas = warp::any().map(move || arenas.clone());
    let games = warp::any().map(move || games.clone());
    let db_filter = warp::any().map(move || db_clone.clone());

    let (qtx, qrx) = tokio::sync::mpsc::unbounded_channel();
    let queue = warp::any().map(move || qtx.clone());
    debug!("Starting server on port {}", port);

    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(arenas)
        .and(games)
        .and(queue)
        .and(db_filter)
        .map(
            |ws: warp::ws::Ws, arenas: AsyncArenas, games: AsyncGames, queue: AsyncQueue, db: SqlitePool| {
                ws.on_upgrade(|websocket| async {
                    let _ = on_connect(websocket, arenas, games, queue, db).await;
                })
            },
        );

    let db_pool_clone = db_pool.clone();
    let db_pool = warp::any().map(move || db_pool.clone());

    let api = warp::path("api")
        .and(warp::post())
        .and(api::json_body())
        .and(db_pool)
        .and_then(api::load_game);

    let service = websocket.or(api);

    tokio::spawn(queue_processer(db_pool_clone, qrx));
    warp::serve(service).run(([0, 0, 0, 0], port)).await;
}

pub async fn send_message(
    message: GlobalServerResponse,
    sink: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
) {
    let message = serde_json::to_string(&message).unwrap();
    let message = warp::ws::Message::text(message);
    let result = sink.send(message).await;
    if let Err(error) = result {
        error!("Error sending message: {:?}", error);
    }
}

pub fn get_arena_state(id: &Uuid, arenas: &AsyncArenas) -> Option<ArenaState> {
    let arenas = arenas.lock().expect("Could not get arena lock");
    let state = arenas.get(id);
    match state {
        Some(state) => Some(state.clone()),
        None => None,
    }
}
pub async fn on_connect(
    ws: warp::ws::WebSocket,
    arenas: AsyncArenas,
    games: AsyncGames,
    queue: AsyncQueue,
    db: SqlitePool,
) -> Result<(), String> {
    // Outgoing messages from this server and incoming messages from the client
    let (mut outgoing_messages, mut incoming_messages) = ws.split();

    let id = database::generate_new_id(&db).await;
    let slug = database::load_slug_default(&db, id).await;
    info!("Client with id {} connected, slug : {}", id, slug);

    let state = ArenaState {
        authenticated: false,
        initialized: false,
        id,
        num_successful_updates: 0,
    };

    arenas
        .lock()
        .expect("Could not get arena lock")
        .insert(id, state);

    let duration = std::time::Duration::from_secs(5 * 60);
    while let Ok(message) = timeout(duration, incoming_messages.next()).await {
        trace!("Client with slug {} sent message", slug);
        // Check if the message timed out (returns None)
        if message.is_none() {
            let outgoing_message = GlobalServerResponse::Timeout;
            send_message(outgoing_message, &mut outgoing_messages).await;
            info!("Client with slug {} timed out", slug);
            break;
        }
        let message = message.unwrap();

        // Then check if the message is not an error
        let message = match message {
            Ok(message) => message,
            Err(_) => {
                let error = "Invalid message, error recieved".to_string();
                let outgoing_message = GlobalServerResponse::Error(error);
                send_message(outgoing_message, &mut outgoing_messages).await;
                continue;
            }
        };

        // Then attempt to parse the message
        let message = message.to_str().unwrap();
        let parsed_message = serde_json::from_str::<ArenaRequest>(&message);
        info!("Client with slug {} sent message", slug);
        trace!("Client with slug {} sent message: {:?}", slug, parsed_message);

        let parsed_message = match parsed_message {
            Ok(parsed_message) => parsed_message,
            Err(_) => {
                let error = "Invalid message".to_string();
                let outgoing_message = GlobalServerResponse::Error(error);
                send_message(outgoing_message, &mut outgoing_messages).await;
                continue;
            }
        };

        // Then collect the internal state and attempt to transition the state using
        // the client's request
        let mut state = get_arena_state(&id, &arenas).expect("Could not get arena state");
        debug!("{} state: {:?}", slug, state);
        let response =
            state_transition(&mut state, games.clone(), queue.clone(), &parsed_message, &db).await;

        // Finally, report the response to the client
        match response {
            Ok(response) => {
                send_message(response, &mut outgoing_messages).await;
            }
            Err(error) => {
                let response = GlobalServerResponse::Error(error);
                send_message(response, &mut outgoing_messages).await;
            }
        }
        debug!("response sent to client {}", slug);

        // And update the state
        debug!("updating state... {}", slug);
        arenas
            .lock()
            .expect("Could not get arena lock")
            .insert(id, state);
        debug!("state updated {}", slug);
    }

    // TODO: any cleanup logic here
    warn!("Client with slug {} disconnected", slug);
    Err(format!("Client with slug {} disconnected", slug))
}

pub fn verify(_secret: &str) -> bool {
    // TODO: add authentication logic here
    return true;
}

pub fn handle_debug_message(message: &str) {
    println!("Debug message: {}", message);
}

pub fn handle_authenticate(
    secret: &str,
    state: &mut ArenaState,
) -> Result<GlobalServerResponse, String> {
    if verify(secret) {
        state.authenticated = true;
        Ok(GlobalServerResponse::Authenticated(Authenticated::Success))
    } else {
        let reason = "Invalid secret".to_string();
        Ok(GlobalServerResponse::Authenticated(
            Authenticated::Failure { reason },
        ))
    }
}

// TODO remove db from this function and let queue handle slugging
pub async fn handle_init(
    state: &mut ArenaState,
    queue: AsyncQueue,
    info: &SmallClientInfo,
    db: &SqlitePool,
) -> Result<GlobalServerResponse, String> {
    let mut game_ledger = Vec::new();

    let update = GameUpdate {
        update_num: 0,
        info: info.clone(),
    }; 
    game_ledger.push(update);
    let id = state.id.clone();

    // TODO: perhaps we'll need a flush_queue function that 
    // waits for the slug to be applied, something about allowing
    // this function to know the db state gives me the heebie jeebies
    update_queue(id, &game_ledger, &mut queue.clone());
    let slug = database::load_slug_default(db, id).await;

    state.initialized = true;

    return Ok(GlobalServerResponse::Initialized(Initialized::Success {
        id: id.to_string(),
        url: format!("{}/demo/{}", HOST_NAME, slug),
    }));
}

pub fn handle_reconnect(
    id: &str,
    state: &mut ArenaState,
    games: AsyncGames,
) -> Result<GlobalServerResponse, String> {
    let id = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            let reason = "Invalid id".to_string();
            return Ok(GlobalServerResponse::Reconnected(Reconnected::Failure {
                reason,
            }));
        }
    };

    if games
        .lock()
        .expect("Could not get game lock")
        .contains_key(&id)
    {
        state.id = id.clone();
        let num_successful_updates = games
            .lock()
            .expect("Could not get game lock")
            .get(&id)
            .unwrap()
            .len();
        state.num_successful_updates = num_successful_updates;
        return Ok(GlobalServerResponse::Reconnected(Reconnected::Success));
    } else {
        let reason = "Client id does not exist".to_string();
        return Ok(GlobalServerResponse::Reconnected(Reconnected::Failure {
            reason,
        }));
    }
}

pub fn handle_game_update(
    state: &mut ArenaState,
    queue: AsyncQueue,
    updates: &Vec<GameUpdate>,
) -> Result<GlobalServerResponse, String> {
    update_queue(state.id.clone(), updates, &mut queue.clone());
    Ok(GlobalServerResponse::Info(
        "Received Game Update".to_string(),
    ))
}

pub fn handle_game_over(
    _state: &mut ArenaState,
    _queue: AsyncQueue,
) -> Result<GlobalServerResponse, String> {
    // TODO: indicate game over somehow
    Ok(GlobalServerResponse::Info("Received Game Over".to_string()))
}

/// Attempts to update some state based on the request
pub async fn state_transition(
    state: &mut ArenaState,
    games: AsyncGames,
    queue: AsyncQueue,
    request: &ArenaRequest,
    db: &SqlitePool,
) -> Result<GlobalServerResponse, String> {
    // If the client is not authenticated, they can only authenticate
    match (request, state.authenticated) {
        (ArenaRequest::Authenticate { secret }, _) => return handle_authenticate(secret, state),
        (_, false) => {
            let reason = "Client must authenticate first".to_string();
            return Ok(GlobalServerResponse::Authenticated(
                Authenticated::Failure { reason },
            ));
        }

        _ => {}
    };

    // If the client is authenticated, they can send debug messages or heartbeats
    match request {
        ArenaRequest::DebugMessage(message) => {
            handle_debug_message(message);
            return Ok(GlobalServerResponse::Info(
                "Received Debug Message".to_string(),
            ));
        }
        ArenaRequest::Heartbeat => {
            return Ok(GlobalServerResponse::Info("Received Heartbeat".to_string()));
        }
        _ => {}
    };

    // An authenticated client can always reconnect if that id exists
    match request {
        ArenaRequest::Reconnect { id } => return handle_reconnect(id, state, games),
        _ => {}
    };

    // If the client is authenticated, they can initialize the game,
    // if not already initialized
    match (request, state.initialized) {
        (ArenaRequest::InitializeGame { info }, false) => return handle_init(state, queue, info, db).await,
        (_, false) => {
            let reason = "Game must be initialized first".to_string();
            return Ok(GlobalServerResponse::Initialized(Initialized::Failure {
                reason,
            }));
        }
        _ => {}
    };

    if !state.initialized || !state.authenticated {
        return Err("Client must be authenticated and initialized at this point".to_string());
    }

    // If the client is authenticated and initialized, they can send updates
    match request {
        ArenaRequest::GameUpdates(updates) => {
            return handle_game_update(state, queue, updates);
        }
        ArenaRequest::GameOver { total_updates: _ } => {
            return handle_game_over(state, queue);
        }
        _ => return Err("Invalid request".to_string()),
    };
}
