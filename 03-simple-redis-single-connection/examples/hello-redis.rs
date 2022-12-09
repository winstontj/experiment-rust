use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {

    let redis_port = 6379;
    let redis_host = "127.0.0.1";
    let redis_key = "hello";
    // Open a client connection to mini-redis server
    let mut redis_cliet = client::connect(format!("{redis_host}:{redis_port}")).await?;

    // Set the key-vallue pair hello-world
    redis_cliet.set(redis_key, "world".into()).await?;

    // Print the value from the server
    let rettrieved_value = redis_cliet.get(redis_key).await?;

    // Print the value
    println!("Get the value from server result={:?}", rettrieved_value);

    Ok(())
}
