# Rust SQLX Basic

Connecting SQLX to a database.

- Use sqlx crate with tokio runtime + postgres driver
- Use dotenv crate to read environment variable. (old habit hard to remove. using dotenv far too often in JS development :p )

## How to start

- Prepare a working postgres database
- Setup a `.env` file that contains DB_HOST, DB_PORT, DB_USER, DB_PASSWORD, and DB_DATABASE
- Execute the simple script using `cargo run`
- The script should run without any error if it can load the required env variables and connect to the database