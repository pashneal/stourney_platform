#[cfg(test)]
pub mod tests;
mod websocket;

pub use websocket::*;

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::{debug, error, info, trace, warn};
use splendor_arena::{models::*, SmallClientInfo};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::timeout;
use uuid::Uuid;
use warp::Filter;

use crate::api;
use crate::constants::HOST_NAME;
use crate::{queue as queue_funcs, queue::AsyncQueue};

type GameLedger = Vec<SmallClientInfo>;
// May need a lock-free data structure here
type Games = HashMap<Uuid, GameLedger>;
// Ditto
pub type Arenas = HashMap<Uuid, ArenaState>;

pub type AsyncGames = std::sync::Arc<std::sync::Mutex<Games>>;
pub type AsyncArenas = std::sync::Arc<std::sync::Mutex<Arenas>>;
