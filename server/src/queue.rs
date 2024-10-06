// This module separates the concerns of the database related functions
// and the operations from the API that update or modify the database.
//
// Structured in the following way:
//
//      API/Websocket -> Queue Buffer <-> Database
//
// By doing this, we can make the client-facing API only wait for database
// operations to complete when it is necessary, and increase concurrent
// throughput when it is not.
//
// Additionally, it makes the api layer far easier to test reliably, without
// needing to mock the database layer, as we can just test the contents
// of the unprocessed queue.
//
// TODO: reading from the database may need to also be added to the queue
// to prevent phantom reads

use log::debug;
use splendor_arena::models::*;
use sqlx::sqlite::SqlitePool;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use crate::database;

// TODO: may want to consider changing the data structure in the following cases:
//  - horizontal scalability is a concern : swap this with Redis
//  - mutexes are too slow : use a lock-free work stealing data structure
//  - throughput is too low or buffer pressure is too high : optimize the incoming GameUpdate
pub type AsyncQueue = UnboundedSender<QueueUpdate>;

pub enum QueueUpdate {
    AddGameInfo {
        id: Uuid,
        update: GameUpdate,
    },

    GetSlug {
        id: Uuid,
        callback: UnboundedSender<String>,
    },

    GenerateId {
        callback: UnboundedSender<Uuid>,
    },

    SetGameOver {
        id: Uuid,
    },
}

/// Process the queue of updates, calling process_update()
/// on each of them and blocking while the receiver still has active senders
pub async fn queue_processer(db_pool: SqlitePool, mut receiver: UnboundedReceiver<QueueUpdate>) {
    loop {
        match receiver.recv().await {
            Some(queue_update) => {
                process_update(&db_pool, queue_update).await;
            }
            None => {
                debug!("[-] Shutting down queue processor, no more senders online.");
                break;
            }
        }
    }
}

async fn process_update(db_pool: &SqlitePool, update: QueueUpdate) {
    match update {
        QueueUpdate::AddGameInfo { id, update } => {
            debug!("[+] Processing game update for {}", id);
            database::simple_save_game_update(db_pool, update, id).await;
        }
        QueueUpdate::GetSlug {
            id,
            callback: channel,
        } => {
            debug!("[+] Processing get slug update for {}", id);
            let slug = database::load_slug_default(db_pool, id).await;
            let _ = channel.send(slug);
        }
        QueueUpdate::GenerateId { callback: channel } => {
            debug!("[+] Processing generate id update");
            let id = database::generate_new_id(db_pool).await;
            let _ = channel.send(id);
        }
        QueueUpdate::SetGameOver { id } => {
            debug!("[+] Processing set game over update for {}", id);
            todo!("unsure what to do in the event of game over");
        }
    }
}

/// Create a new id for a game, blocks until the id is created
pub async fn create_id(sender: &UnboundedSender<QueueUpdate>) -> Uuid {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let _ = sender.send(QueueUpdate::GenerateId { callback: tx });
    rx.recv().await.unwrap()
}

/// Get the slug for a given id from the database,
/// or generate a new one if it does not exist,
/// blocks until the slug is retrieved
pub async fn get_slug(id: Uuid, sender: &UnboundedSender<QueueUpdate>) -> String {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let _ = sender.send(QueueUpdate::GetSlug { id, callback: tx });
    rx.recv().await.unwrap()
}

/// Append a game update to the queue, and returns
/// immediately
pub fn push_game_updates(
    id: Uuid,
    updates: &Vec<GameUpdate>,
    sender: &mut UnboundedSender<QueueUpdate>,
) {
    for update in updates {
        let _ = sender.send(QueueUpdate::AddGameInfo {
            id,
            update: update.clone(),
        });
    }
}
