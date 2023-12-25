use bytes::BytesMut;
use log::info;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::memory::Memory;
use crate::request::parse_request;
use crate::{commands, response};

pub type MemoryArc = Arc<Mutex<Memory>>;

const DEFAULT_RESPONSE: &[u8; 7] = b"+PONG\r\n";
const BUF_SIZE: usize = 1024;

pub struct Session {
    client_id: SocketAddr,
    memory: MemoryArc,
    socket: TcpStream,
}

pub fn create_session(socket: TcpStream, client_id: SocketAddr, memory: MemoryArc) -> Session {
    Session {
        client_id,
        socket,
        memory,
    }
}

pub async fn handle_session(mut session: Session) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "This is a connection from {}",
        session.client_id.to_string()
    );

    // initialize buffer & get socket + client_id string
    let comm_factory = commands::CommandFactory {
        memory: &mut session.memory,
    };
    let mut buffer = BytesMut::with_capacity(BUF_SIZE);
    let mut resp_buffer = BytesMut::with_capacity(BUF_SIZE);
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
        let req = parse_request(&buffer[..]);

        {
            // process request
            let comm = comm_factory.create_command(&req);

            // execute command
            let resp = comm.execute();

            response::serialize_resp(resp, &mut resp_buffer);
        }

        // respond to client
        socket.write(&resp_buffer).await?;

        // clear buffer
        buffer.clear();
        resp_buffer.clear();
    }

    Ok(())
}
