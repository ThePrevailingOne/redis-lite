mod session;

use tokio::net::TcpListener;

const LOCALHOST_ADDR: &str = "127.0.0.1:6379";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Let's start!");

    let listener = TcpListener::bind(LOCALHOST_ADDR).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        let session = session::create_session(socket, addr);
        println!("Got a client!");
        session::handle_session(session).await?;
    }
}
