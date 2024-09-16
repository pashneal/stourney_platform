// allows Websocket::split()
use warp::Filter;
use futures_util::{stream::SplitSink, SinkExt, StreamExt, TryFutureExt};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use splendor_arena::{ClientInfo, models::*};
use std::collections::VecDeque;
use uuid::Uuid;
use tokio::time::timeout;
use log::{info, error, warn, trace};

// TODO: may want to consider changing the data structure in the following cases:
//  - horizontal scalability is a concern : swap this with Redis
//  - mutexes are too slow : use a lock-free work stealing data structure
//  - throughput is too low or buffer pressure is too high : optimize the incoming GameUpdate
type Queue = VecDeque<Vec<GameUpdate>>;

type GameLedger = Vec<ClientInfo>;
// May need a lock-free data structure here
type Games = HashMap<Uuid, GameLedger>;
// Ditto
type Arenas = HashMap<Uuid, ArenaState>;

pub type AsyncQueue = std::sync::Arc<std::sync::Mutex<Queue>>;
pub type AsyncGames = std::sync::Arc<std::sync::Mutex<Games>>;
pub type AsyncArenas = std::sync::Arc<std::sync::Mutex<Arenas>>;

#[derive(Debug, Clone)]
pub struct ArenaState {
    pub authenticated: bool,           // Did the client authenticate?
    pub initialized: bool,             // Did the client initialize the game?
    pub id: Uuid,                      // The client's id set on websocket connect
    pub num_successful_updates: usize, // Number of successful updates since init
}

pub fn generate_new_id(arenas: &Arenas) -> Uuid {
    let mut id = Uuid::new_v4();
    while arenas.contains_key(&id) {
        id = Uuid::new_v4();
    }
    return id
}


pub async fn serve(port: u16){
    let arenas = std::sync::Arc::new(std::sync::Mutex::new(Arenas::new()));
    let games = std::sync::Arc::new(std::sync::Mutex::new(Games::new()));
    let queue = std::sync::Arc::new(std::sync::Mutex::new(Queue::new()));
    
    let arenas = warp::any().map(move || arenas.clone());
    let games = warp::any().map(move || games.clone());
    let queue = warp::any().map(move || queue.clone());

    let route = warp::path("ws")
        .and(warp::ws())
        .and(arenas)
        .and(games)
        .and(queue)
        .map(|ws: warp::ws::Ws, arenas : AsyncArenas, games: AsyncGames, queue: AsyncQueue| {
            ws.on_upgrade(|websocket| async {
                let _ = on_connect(websocket, arenas, games, queue).await;
            })
        });

    warp::serve(route).run(([127, 0, 0, 1], port)).await;
}
pub async fn send_message(message: GlobalServerResponse, sink: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>) {
    let message = serde_json::to_string(&message).unwrap();
    let message = warp::ws::Message::text(message);
    let result =  sink.send(message).await;
    if let Err(error) = result {
        error!("Error sending message: {:?}", error);
    }
}

pub fn get_arena_state(id: &Uuid, arenas: &AsyncArenas) -> Option<ArenaState> {
    let arenas = arenas.lock().expect("Could not get arena lock");
    let state = arenas.get(id);
    match state {
        Some(state) => Some(state.clone()),
        None => None
    }
}
pub async fn on_connect(ws : warp::ws::WebSocket, arenas: AsyncArenas, games: AsyncGames, queue: AsyncQueue) -> Result<(), String>{
    // Outgoing messages from this server and incoming messages from the client
    let (mut outgoing_messages, mut incoming_messages) = ws.split();

    let id = generate_new_id(&arenas.lock().expect("Could not get arena lock"));
    let state = ArenaState {
        authenticated: false,
        initialized: false,
        id,
        num_successful_updates: 0,
    };
    

    info!("Client with id {} connected", id);
    arenas.lock().expect("Could not get arena lock").insert(id, state);

    let duration = std::time::Duration::from_secs(5*60);
    while let Ok(message) = timeout(duration, incoming_messages.next()).await {

        trace!("Client with id {} sent message", id);
        // Check if the message timed out (returns None)
        if message.is_none() {
            let outgoing_message = GlobalServerResponse::Timeout;
            send_message(outgoing_message, &mut outgoing_messages).await;
            info!("Client with id {} timed out", id);
            break
        }
        let message = message.unwrap();


        // Then check if the message is not an error
        let message = match message {
            Ok(message) => message,
            Err(_) => {
                let error = "Invalid message, error recieved".to_string();
                let outgoing_message = GlobalServerResponse::Error(error);
                send_message(outgoing_message, &mut outgoing_messages).await;
                continue
            }
        };

        // Then attempt to parse the message
        let message = message.to_str().unwrap();
        let parsed_message = serde_json::from_str::<ArenaRequest>(&message);
        info!("Client with id {} sent message: {:?}", id, parsed_message);

        let parsed_message = match parsed_message {
            Ok(parsed_message) => parsed_message,
            Err(_) => {
                let error = "Invalid message".to_string();
                let outgoing_message = GlobalServerResponse::Error(error);
                send_message(outgoing_message, &mut outgoing_messages).await;
                continue
            }
        };

        // Then collect the internal state and attempt to transition the state using
        // the client's request
        info!("attempting to transition state...");
        let mut state = get_arena_state(&id, &arenas).expect("Could not get arena state");
        info!("state: {:?}", state);
        let response = state_transition(&mut state, games.clone(), queue.clone(), &parsed_message).await;

        // Finally, report the response to the client
        info!("sending response to client...");  
        match response {
            Ok(response) => {
                send_message(response, &mut outgoing_messages).await;
            },
            Err(error) => {
                let response = GlobalServerResponse::Error(error);
                send_message(response, &mut outgoing_messages).await;
            }
        }
        info!("response sent to client");

        // And update the state
        info!("updating state...");
        arenas.lock().expect("Could not get arena lock").insert(id, state);
        info!("state updated");
    }

    // TODO: any cleanup logic here
    warn!("Client with id {} disconnected", id);
    Err(format!("Client with id {} disconnected", id))
}

pub fn verify(secret: &str) -> bool {
    // TODO: add authentication logic here
    return true
}

pub fn handle_debug_message(message: &str) {
    println!("Debug message: {}", message);
}

pub fn handle_authenticate(secret: &str, state: &mut ArenaState) -> Result<GlobalServerResponse, String> {
    if verify(secret) {
        state.authenticated = true;
        Ok(GlobalServerResponse::Authenticated(Authenticated::Success))
    } else {
        let reason = "Invalid secret".to_string();
        Ok(GlobalServerResponse::Authenticated(Authenticated::Failure{ reason }))
    }
}

// TODO: ensure proper locking write to prevent race conditions
pub fn  handle_init(state: &mut ArenaState, games: AsyncGames, info: &ClientInfo) -> Result<GlobalServerResponse, String> {
    state.initialized = true;
    {
        let mut games = games.lock().expect("Could not get game lock");
        games.insert(state.id.clone(), Vec::new());
        let mut game_ledger = Vec::new();
        game_ledger.push(info);
        

    }
    let id = state.id.clone();
    return Ok(GlobalServerResponse::Initialized(Initialized::Success { id : id.to_string() }))
}

pub fn handle_reconnect(id: &str, state: &mut ArenaState, games: AsyncGames) -> Result<GlobalServerResponse, String> {
    let id = match Uuid::parse_str(id) {
        Ok(id) => {id},
        Err(_) => {
            let reason = "Invalid id".to_string();
            return Ok(GlobalServerResponse::Reconnected(Reconnected::Failure{ reason }))
        }
    };

    if games.lock().expect("Could not get game lock").contains_key(&id) {
        state.id = id.clone();
        let num_successful_updates = games.lock().expect("Could not get game lock").get(&id).unwrap().len();
        state.num_successful_updates = num_successful_updates;
        return Ok(GlobalServerResponse::Reconnected(Reconnected::Success))
    } else {
        let reason = "Client id does not exist".to_string();
        return Ok(GlobalServerResponse::Reconnected(Reconnected::Failure{ reason }))
    }   
}

pub fn handle_game_update(state: &mut ArenaState, queue: AsyncQueue, updates: &Vec<GameUpdate>) -> Result<GlobalServerResponse, String> {
    let mut queue = queue.lock().expect("Could not get queue lock");
    queue.push_back(updates.clone());
    Ok(GlobalServerResponse::Info("Received Game Update".to_string()))
}

pub fn handle_game_over(state: &mut ArenaState, queue: AsyncQueue) -> Result<GlobalServerResponse, String> {
    // TODO: indicate game over somehow
    Ok(GlobalServerResponse::Info("Received Game Over".to_string()))
}

/// Attempts to update some state based on the request
pub async fn state_transition(state: &mut ArenaState, games: AsyncGames, queue : AsyncQueue, request: &ArenaRequest) -> Result<GlobalServerResponse, String> {

    match request {
        ArenaRequest::DebugMessage(message) => {
            handle_debug_message(message);
            return Ok(GlobalServerResponse::Info("Received Debug Message".to_string()));
        }
        ArenaRequest::Heartbeat => {
            return Ok(GlobalServerResponse::Info("Received Heartbeat".to_string()));
        }
        _ => {}
    };

    match (request, state.authenticated) {
        (ArenaRequest::Authenticate { secret }, _) =>  {
            return handle_authenticate(secret, state)
        }
        (_, false) =>   {
            let reason = "Client must authenticate first".to_string();
            return Ok(GlobalServerResponse::Authenticated(Authenticated::Failure{ reason }))
        }

        _ => {}
    };

    // An authenticated client can always reconnect if that id exists
    match request {
        ArenaRequest::Reconnect { id } => {
            return handle_reconnect(id, state, games)
        }
        _ => {}
    };


    // If the client is authenticated, they can initialize the game,
    // if not already initialized
    match (request, state.initialized) {
        (ArenaRequest::InitializeGame{ info }, false) => {
            return handle_init(state, games, info)
        }
        (_, false) => {
            let reason = "Game must be initialized first".to_string();
            return Ok(GlobalServerResponse::Initialized(Initialized::Failure{ reason }))
        }
        _ => {}
    };  

    if !state.initialized || !state.authenticated {
        return Err("Client must be authenticated and initialized at this point".to_string())
    }

    // If the client is authenticated and initialized, they can send updates
    match request {
        ArenaRequest::GameUpdates(updates) => {
            return handle_game_update(state, queue, updates);
        },
        ArenaRequest::GameOver{ total_updates : _} => {
            return handle_game_over(state, queue);
        },
        _ => return Err("Invalid request".to_string())
    };
}
