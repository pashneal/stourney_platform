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

pub type QueueUpdate = (Uuid, GameUpdate);

// TODO: See above todo may want to consider changing the way the queue is processed
// if the queue is too slow
pub async fn queue_processer(db_pool: SqlitePool, mut receiver: UnboundedReceiver<QueueUpdate>) {
    loop {
        match receiver.recv().await {
            Some(game_update) => {
                let (uuid, update) = game_update;
                println!("[+] Processing client update for {}", uuid);
                database::simple_save_game_update(&db_pool, update, uuid).await;
            }
            None => {
                println!("[-] No more clients to process");
                break;
            }
        }
    }
}

pub fn update_queue(
    id: Uuid,
    updates: &Vec<GameUpdate>,
    sender: &mut UnboundedSender<QueueUpdate>,
) {
    for update in updates {
        let _ = sender.send((id, update.clone()));
    }
}
