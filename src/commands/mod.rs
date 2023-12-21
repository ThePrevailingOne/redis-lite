mod ping;

use log::warn;

use crate::{resp::*, session::MemoryArc};

pub trait Command {
    fn execute(&self) -> RESPData;
}

pub struct CommandFactory<'cf> {
    pub memory: &'cf mut MemoryArc,
}

impl CommandFactory<'_> {
    pub fn create_command(&self, data: RESPData) -> Box<dyn Command> {
        if !matches!(data, RESPData::Array(_)) {
            warn!("RESP data received not array");
        }

        let mut b = Box::new(ping::Ping {});
        if let RESPData::Array(a) = data {
            let comm_key = &a.children[0];

            let comm_key = bytes_from_bulk_string(comm_key);

            b = match comm_key.to_ascii_uppercase().as_slice() {
                b"PING" => Box::new(ping::Ping {}),
                _ => Box::new(ping::Ping {}),
            }
        }
        b
    }
}
