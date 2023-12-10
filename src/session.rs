use bytes::BytesMut;
use log::info;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::request::parse_request;

const DEFAULT_RESPONSE: &[u8; 7] = b"+PONG\r\n";
const BUF_SIZE: usize = 1024;

pub struct Session {
    client_id: SocketAddr,
    socket: TcpStream,
}

pub fn create_session(socket: TcpStream, client_id: SocketAddr) -> Session {
    Session { client_id, socket }
}

pub async fn handle_session(session: Session) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "This is a connection from {}",
        session.client_id.to_string()
    );

    // initialize buffer & get socket + client_id string
    let mut buffer = BytesMut::with_capacity(BUF_SIZE);
    let mut socket = session.socket;

    // read tcpstream input
    loop {
        let bytes_read = socket.read_buf(&mut buffer).await?;

        // if bytes_read is zero, then connection should be closed
        if bytes_read == 0 {
            info!("Connection from {} closed", session.client_id.to_string());
            break;
        }

        info!(
            "Client from {} send {} bytes: {:?}",
            session.client_id.to_string(),
            bytes_read,
            &buffer
        );

        // parse request
        parse_request(&buffer[..]);

        // process request

        // respond to client
        socket.write(&DEFAULT_RESPONSE[..]).await?;

        // clear buffer
        buffer.clear();
    }

    Ok(())
}
