use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
//#[async_std::main] // Requires the `attributes` feature of `async-std`
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {


    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("test.db")
            .create_if_missing(true)
    );

    let pool = pool.await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("Row: {:?}", row);
    assert_eq!(row.0, 150);

    Ok(())
}
