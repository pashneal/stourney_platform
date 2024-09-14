use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
use futures::TryStreamExt;
use sqlx::Row;
use splendor_arena::Arena;

mod websocket;

/// Note: this uses sqlx compile time checker
/// to ensure that the queries are correct
/// be sure to run sqlx prepare if strange errors occur
/// also add your DATABASE_URL is set in the .env file

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    env_logger::init();
    
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;


    let email = "pashneal@gmail.com";
    let mut rows = sqlx::query!("SELECT email FROM users WHERE email = ?", email)
        .fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        print!("row: {:?}", row);
    }
    
    websocket::test(3031).await;



    Ok(())
}

/// The function that is called when the database is started
/// responsible for setting up the database to get high performance
/// speeds
pub async fn on_start_up(pool: &SqlitePool) {
    // Set option to use write ahead logging 
    // which means multiple concurrent readers even
    // during open write transactions
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(pool)
        .await
        .expect("Failed to set journal mode to WAL");

    // Since we are using WAL mode, we
    // don't need synchronous writes as WAL guarantees
    // consistency in synchronous = normal mode
    sqlx::query("PRAGMA synchronous = NORMAL")
        .execute(pool)
        .await
        .expect("Failed to set synchronous mode to NORMAL");

    // Store temporary tables and files in memory,
    // we will lose data if the database is closed but 
    // this is a trade off for speed
    sqlx::query("PRAGMA temp_store = MEMORY")
        .execute(pool)
        .await
        .expect("Failed to set temp store to MEMORY");
    

    // Use memory mapped I/O for reading and writing
    // as it can be faster than normal I/O
    // note this has implications for I/O errors on sqlite
    sqlx::query("PRAGMA mmap_size = 30_000_000_000")
        .execute(pool)
        .await
        .expect("Failed to set mmap size to 256MB");
}


pub async fn init_db() -> Result<(), sqlx::Error>{
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    on_start_up(&pool).await;
    Ok(())
}
