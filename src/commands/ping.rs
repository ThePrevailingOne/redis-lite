use bytes::Bytes;
use log::info;

use crate::resp::{RESPData, SimpleRESP};

use super::Command;

pub struct Ping {}

impl Command for Ping {
    fn execute(&self) -> RESPData {
        info!("PING is called!");

        let bytes = Bytes::from_static(b"PONG");
        let simple = SimpleRESP { value: bytes };
        RESPData::SimpleString(simple)
    }
}

impl Ping {
    pub fn new() -> Box<dyn Command> {
        Box::new(Ping {})
    }
}
