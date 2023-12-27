use core::panic;

use bytes::Bytes;

use crate::{
    resp::{bytes_from_bulk_string, AggregateRESP, RESPData, SimpleRESP},
    session::MemoryArc,
};

use super::Command;

pub struct Get<'a> {
    mem: &'a MemoryArc,
    key: Bytes,
}

impl Command for Get<'_> {
    fn execute(&self) -> RESPData {
        let mem = self.mem.lock().unwrap();
        let frame = mem.get(self.key.clone());
        RESPData::SimpleString(SimpleRESP {
            value: Bytes::from(frame.string_value()),
        })
    }
}

impl<'a> Get<'a> {
    pub fn new(data: &'a RESPData, mem: &'a MemoryArc) -> Box<dyn Command + 'a> {
        match data {
            RESPData::Array(AggregateRESP { children }) => {
                let mut children = children.range(1..);
                let key = bytes_from_bulk_string(children.next().expect("key expected!"));
                Box::new(Self { key, mem })
            }
            _ => panic!("RESP data received not array!"),
        }
    }
}
