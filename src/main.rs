mod memory;
mod request;
mod session;

use std::sync::{Arc, Mutex};

use env_logger;
use log::info;
use tokio::net::TcpListener;

const LOCALHOST_ADDR: &str = "127.0.0.1:6379";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logger
    env_logger::init();
    info!("Redis Server started!");

    let listener = TcpListener::bind(LOCALHOST_ADDR).await?;

    let mem = memory::Memory::new();
    let mem = Arc::new(Mutex::new(mem));

    loop {
        let (socket, addr) = listener.accept().await?;

        let mem = mem.clone();

        tokio::spawn(async move {
            let session = session::create_session(socket, addr, mem);
            session::handle_session(session).await;
        });
    }
}
