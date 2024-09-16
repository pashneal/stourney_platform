use splendor_arena::models::GameUpdate;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use uuid::Uuid;

/// Connects to the database and returns a pool
/// Requires the DATABASE_URL environment variable is set
pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // We are using Sqlite, so it is recommended to have only one connection
    // TODO: conventional wisdom may fail in this instance, so perhaps tweak later
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    println!("Connected to database");
    init_schema(&pool).await;
    sqlite_startup(&pool).await;
    Ok(pool)
}

/// The function that is called when the database is started
/// responsible for setting up the database to get high performance
/// speeds on sqlite
pub async fn sqlite_startup(pool: &SqlitePool) {
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
    sqlx::query("PRAGMA mmap_size = 30000000000")
        .execute(pool)
        .await
        .expect("Failed to set mmap size to 256MB");

    // Must turn on support for foreign keys constraint
    // as it is off by default on sqlite
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await
        .expect("Failed to turn on foreign keys");
}

/// Initializes the schema for the database, be sure that
/// the schema.sql file is in the same directory as src
pub async fn init_schema(pool: &SqlitePool) {
    let schema = include_str!("schema.sql");
    sqlx::query(&schema)
        .execute(pool)
        .await
        .expect("Failed to create schema");
}

/// Serializes the game update and saves it to the database,
/// "simple" because this doesn't make the info very queryable, 
/// but is good enough for storing data
pub async fn simple_save_game_update(pool: &SqlitePool, game_update: GameUpdate, uuid: Uuid) {
    let uuid = uuid.to_string();
    let turnid = game_update.update_num as i32;

    let game_exists = sqlx::query!("SELECT gameuuid FROM simplegames WHERE gameuuid = ? AND turnid = ?", uuid, turnid)
        .fetch_all(pool)
        .await
        .expect("Failed to query database");

    let game_update = serde_json::to_string(&game_update).unwrap();
    if game_exists.len() == 0{
        sqlx::query!("INSERT INTO simplegames (gameuuid, turnid, gameupdate) VALUES (?, ?, ?)", uuid, turnid, game_update)
            .execute(pool)
            .await
            .expect("Failed to insert game update");
    } else {
        sqlx::query!("UPDATE simplegames SET gameupdate = ? WHERE gameuuid = ? AND turnid = ?", game_update, uuid, turnid)
            .execute(pool)
            .await
            .expect("Failed to update game update");
    }

        

}
