use bytes::Bytes;

use crate::resp::{RESPData, SimpleRESP};

use super::Command;

pub struct NotFound {
    msg: String,
}

impl Command for NotFound {
    fn execute(&self) -> RESPData {
        RESPData::SimpleError(SimpleRESP {
            value: Bytes::copy_from_slice(self.msg.as_str().as_bytes()),
        })
    }
}

impl NotFound {
    pub fn new(s: &str) -> Box<dyn Command> {
        Box::new(Self { msg: s.to_string() })
    }
}
