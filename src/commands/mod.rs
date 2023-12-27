mod echo;
mod get;
mod not_found;
mod ping;
mod set;

use log::warn;

use crate::{resp::*, session::MemoryArc};

pub trait Command {
    fn execute(&self) -> RESPData;
}

pub struct CommandFactory<'cf> {
    pub memory: &'cf mut MemoryArc,
}

impl<'cf> CommandFactory<'cf> {
    pub fn create_command(&'cf self, data: &'cf RESPData) -> Box<dyn Command + 'cf> {
        if !matches!(data, RESPData::Array(_)) {
            warn!("RESP data received not array");
        }

        if let RESPData::Array(a) = data {
            let comm_key = &a.children[0];

            let comm_key = bytes_from_bulk_string(comm_key);

            match comm_key.to_ascii_uppercase().as_slice() {
                b"PING" => ping::Ping::new(),
                b"ECHO" => echo::Echo::new(&data),
                b"SET" => set::Set::new(&data, self.memory),
                _ => not_found::NotFound::new("Command not found"),
            }
        } else {
            not_found::NotFound::new("Not a proper command")
        }
    }
}
