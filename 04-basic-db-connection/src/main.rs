use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Load up dotenv so it will set variable to environment variable
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set.");
    let db_port = env::var("DB_PORT").expect("DB_POST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set.");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
    let db_database = env::var("DB_DATABASE").expect("DB_DATABASE must be set.");
    let db_connection_string =
        format!("postgress://{db_user}:{db_password}@{db_host}:{db_port}/{db_database}");

    let db_pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_connection_string)
        .await?;

    let query_result: (i32,) = sqlx::query_as("SELECT $1")
        .bind(278_320)
        .fetch_one(&db_pool)
        .await?;

    assert_eq!(query_result.0, 278320);

    Ok(())
}
