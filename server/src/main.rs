mod database;
mod queue;
mod websocket;

/// Note: this uses sqlx compile time checker
/// to ensure that the queries are correct
/// be sure to run sqlx prepare if strange errors occur
/// also add your DATABASE_URL is set in the .env file

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env_logger::init();
    let db = database::connect().await?;
    websocket::serve(3031, db).await;
    Ok(())
}
