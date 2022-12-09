use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

async fn process_stream(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // create a new instance of HashMap
    let mut db = HashMap::new();

    // Create a new connection that is able to perform read/write operation.
    // This connection is determined by mini_redis. The implementation will be different if using other library
    let mut connection = Connection::new(socket);

    while let Some(received_frame) = connection.read_frame().await.unwrap() {
        println!("Socket GOT {:?}", received_frame);
        // Just reply with error
        let response = match Command::from_frame(received_frame).unwrap() {
            Set(cmd) => {
                // Store the cmd to database
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unsupported {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let redis_port = 6379;
    let redis_host = "127.0.0.1";
    // Bind the listener to a port
    let redis_listener = TcpListener::bind(format!("{redis_host}:{redis_port}")).await.unwrap();
    loop {
        // Get the socker stream that listen to the port
        let (socket, _) = redis_listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process_stream(socket).await;
        });
    }
}
