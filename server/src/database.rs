use crate::slug_list::{ADJECTIVES, NOUNS};
use log::{debug, info, trace};
use rand::Rng;
use splendor_arena::models::GameUpdate;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
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
    info!("Connected to database!");
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

/// Generates a new unique game id and saves it to the database
pub async fn generate_new_id(pool: &SqlitePool) -> Uuid {
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();

    sqlx::query!("INSERT INTO games (game_uuid) VALUES (?)", uuid_str)
        .execute(pool)
        .await
        .expect("Failed to insert game");

    uuid
}

/// Serializes the game update and saves it to the database,
/// "simple" because this doesn't make the info very queryable,
/// but is good enough for storing data
pub async fn simple_save_game_update(pool: &SqlitePool, game_update: GameUpdate, uuid: Uuid) {
    debug!("[+] Saving game update...");
    let uuid = uuid.to_string();
    let turnid = game_update.update_num as i32;

    let game_exists = sqlx::query!(
        "SELECT update_uuid FROM game_updates WHERE update_uuid = ? AND turn_id = ?",
        uuid,
        turnid
    )
    .fetch_all(pool)
    .await
    .expect("Failed to query database");

    let game_update = serde_json::to_string(&game_update).unwrap();
    if game_exists.len() == 0 {
        sqlx::query!(
            "INSERT INTO game_updates (update_uuid, turn_id, game_update) VALUES (?, ?, ?)",
            uuid,
            turnid,
            game_update
        )
        .execute(pool)
        .await
        .expect("Failed to insert game update");
    } else {
        trace!("[+] Updating existing game update {}", uuid);
        sqlx::query!(
            "UPDATE game_updates SET game_update = ? WHERE update_uuid = ? AND turn_id = ?",
            game_update,
            uuid,
            turnid
        )
        .execute(pool)
        .await
        .expect("Failed to update game update");
    }
}

/// Loads the game update from the database
pub async fn load_game_update(pool: &SqlitePool, uuid: Uuid, turnid: i32) -> Option<GameUpdate> {
    let uuid = uuid.to_string();
    let game_update = sqlx::query!(
        "SELECT game_update FROM game_updates WHERE update_uuid = ? AND turn_id = ?",
        uuid,
        turnid
    )
    .fetch_one(pool)
    .await;
    if let Ok(game_update) = game_update {
        let game_update: Option<String> = game_update.game_update;
        game_update.map(|game_update| {
            serde_json::from_str(&game_update).expect("could not deserialize game update")
        })
    } else {
        None
    }
}
/// Generates a unique slug for a url
/// TODO: this is slow in the case of lots of collisions
pub async fn generate_unique_slug(pool: &SqlitePool) -> String {
    loop {
        let first: usize;
        let second: usize;
        let third: i32;
        {
            let mut rng = rand::thread_rng();
            first = rng.gen_range(0..ADJECTIVES.len());
            second = rng.gen_range(0..NOUNS.len());
            third = rng.gen_range(0..1000);
        }
        let slug = format!("{}_{}{:04}", ADJECTIVES[first], NOUNS[second], third);
        let slug = slug.to_string();

        let slug_exists = sqlx::query!("SELECT slug FROM slugs WHERE slug = ?", slug)
            .fetch_one(pool)
            .await;
        if slug_exists.is_err() {
            return slug;
        }
    }
}

/// Saves a slug to the database
pub async fn save_slug(pool: &SqlitePool, uuid: Uuid, slug: &str) {
    let uuid = uuid.to_string();
    sqlx::query!(
        "INSERT INTO slugs (slug_id, slug) VALUES (?, ?)",
        uuid,
        slug
    )
    .execute(pool)
    .await
    .expect("Failed to insert slug");
}

/// Loads a uuid given a slug form the database
pub async fn load_uuid_from_slug(pool: &SqlitePool, slug: &str) -> Result<Uuid, sqlx::Error> {
    let slug = slug.to_string();
    let uuid = sqlx::query!("SELECT slug_id FROM slugs WHERE slug = ?", slug)
        .fetch_one(pool)
        .await;

    if let Ok(uuid) = uuid {
        let uuid = uuid.slug_id;
        let uuid = uuid.unwrap();
        Ok(Uuid::parse_str(&uuid).expect("Failed to parse uuid"))
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

/// Loads a slug from the database if it is present,
pub async fn load_slug(pool: &SqlitePool, uuid: Uuid) -> Option<String> {
    let uuid = uuid.to_string();
    let slug = sqlx::query("SELECT slug FROM slugs WHERE slug_id = ?")
        .bind(uuid)
        .fetch_one(pool)
        .await;

    if slug.is_err() {
        return None;
    }
    let slug = slug.unwrap();
    let slug = slug.get_unchecked(0);
    Some(slug)
}

/// Loads a slug from the database if it is present,
/// otherwise returns a human readable string to be used as a slug
/// and saves it to the database
pub async fn load_slug_default(pool: &SqlitePool, uuid: Uuid) -> String {
    let slug = load_slug(pool, uuid).await;
    match slug {
        Some(slug) => slug,
        None => {
            let slug = generate_unique_slug(pool).await;
            save_slug(pool, uuid, &slug).await;
            slug
        }
    }
}
