use bytes::{Bytes, BytesMut};

use crate::{
    memory::Frame,
    resp::{bytes_from_bulk_string, AggregateRESP, RESPData, SimpleRESP},
    session::MemoryArc,
};

use super::Command;

pub struct Set<'a> {
    mem: &'a MemoryArc,
    key: Bytes,
    value: Bytes,
}

impl Command for Set<'_> {
    fn execute(&self) -> RESPData {
        let mut mem = self.mem.lock().unwrap();
        mem.set(self.key.clone(), Frame::string_frame(&self.value));
        RESPData::SimpleString(SimpleRESP {
            value: Bytes::copy_from_slice(
                format!(
                    "key '{}' set with value '{}'",
                    String::from_utf8_lossy(&self.key),
                    String::from_utf8_lossy(&self.value)
                )
                .as_bytes(),
            ),
        })
    }
}

impl<'a> Set<'a> {
    pub fn new(data: &'a RESPData, mem: &'a MemoryArc) -> Box<dyn Command + 'a> {
        match data {
            RESPData::Array(AggregateRESP { children: arr }) => {
                let mut arr = arr.range(1..);
                let key = bytes_from_bulk_string(arr.next().expect("key expected!"));
                let value = bytes_from_bulk_string(arr.next().expect("value expected!"));
                Box::new(Self { mem, key, value })
            }
            _ => panic!(),
        }
    }
}
