mod memory;
mod request;
mod session;

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

    loop {
        let (socket, addr) = listener.accept().await?;
        let session = session::create_session(socket, addr);
        session::handle_session(session).await?;
    }
}
