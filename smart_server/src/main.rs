mod handler;
mod smart_socket;

use handler::{Request, RequestHandler};
use smart_socket::SmartSocket;
use std::error::Error;
use stp::server::{StpConnection, StpServer};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr).await?;
    let smart_socket = SmartSocket::default();

    loop {
        let connection = server.accept().await;

        let connection = match connection {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr().await {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        let smart_socket = smart_socket.clone();

        if handle_connection(connection, smart_socket).await.is_err() {
            println!("Client disconnected: {}", addr);
        }
    }
}

async fn handle_connection(
    connection: StpConnection,
    smart_socket: SmartSocket,
) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(smart_socket);
    loop {
        let req_str = connection.recv_request().await?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req)).await?;
    }
}
