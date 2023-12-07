use std::net::SocketAddr;

use tokio::net::TcpStream;

pub struct Session {
    client_id: SocketAddr,
    socket: TcpStream,
}

pub fn create_session(mut socket: TcpStream, client_id: SocketAddr) -> Session {
    return Session { client_id, socket };
}

pub fn handle_session(mut session: Session) {
    println!(
        "This is a connection from {}",
        session.client_id.to_string()
    );
}
