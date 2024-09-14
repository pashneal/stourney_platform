// allows Websocket::split()
use futures_util::stream::StreamExt;
use warp::Filter;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use splendor_arena::{ClientInfo, models::*};
use std::collections::VecDeque;

// TODO: may want to consider changing the data structure in the following cases:
//  - horizontal scalability is a concern : swap this with Redis
//  - mutexes are too slow : use a lock-free work stealing data structure
//  - throughput is too low or buffer pressure is too high : optimize the incoming GameUpdates
pub struct Queue {
    queue: VecDeque<Vec<GameUpdate>>,
}

type GameLedger = Vec<ClientInfo>;
// May need a lock-free data structure here
type Games = HashMap<String, GameLedger>;

pub type AsyncQueue = std::sync::Arc<std::sync::Mutex<Queue>>;
pub type AsyncGames = std::sync::Arc<std::sync::Mutex<Games>>;

#[derive(Debug, Clone)]
pub struct ArenaState {
    pub authenticated: bool, // Did the client authenticate?
    pub initialized: bool,   // Did the client initialize the game?
    pub id: String,          // The client's id set on websocket connect
    pub num_successful_updates: usize, // Number of successful updates since init
}


pub async fn test(port: u16){
    let route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async {
                let (tx, rx) = websocket.split();
                let _ = rx.forward(tx).await;
            })
        });

    warp::serve(route).run(([127, 0, 0, 1], port)).await;
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
    return Ok(GlobalServerResponse::Initialized(Initialized::Success { id }))
}

pub fn handle_reconnect(id: &str, state: &mut ArenaState, games: AsyncGames) -> Result<GlobalServerResponse, String> {
    if games.lock().expect("Could not get game lock").contains_key(id) {
        state.id = id.to_string();
        let num_successful_updates = games.lock().expect("Could not get game lock").get(id).unwrap().len();
        state.num_successful_updates = num_successful_updates;
        return Ok(GlobalServerResponse::Reconnected(Reconnected::Success))
    } else {
        let reason = "Client id does not exist".to_string();
        return Ok(GlobalServerResponse::Reconnected(Reconnected::Failure{ reason }))
    }   
}

pub fn handle_game_update(state: &mut ArenaState, queue: AsyncQueue, updates: &Vec<GameUpdate>) -> Result<GlobalServerResponse, String> {
    let mut queue = queue.lock().expect("Could not get queue lock");
    queue.queue.push_back(updates.clone());
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
            return Ok(GlobalServerResponse::Info("Received".to_string()));
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
        (ArenaRequest::InitializeGame{ info }, true) => {
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
