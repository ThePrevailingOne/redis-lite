use crate::resp::{bytes_from_bulk_string, RESPData, SimpleRESP};
use bytes::BytesMut;
use log::info;

use super::Command;

pub struct Echo<'echo> {
    data: &'echo RESPData,
}

impl Command for Echo<'_> {
    fn execute(&self) -> RESPData {
        info!("ECHO is called!");
        let mut res = BytesMut::new();

        if let RESPData::Array(arr) = self.data {
            let len = &arr.children.len();
            for (i, child) in &mut arr.children.range(..).enumerate() {
                if i == 0 {
                    continue;
                }
                res.extend(bytes_from_bulk_string(&child));
                if i < len - 1 {
                    res.extend_from_slice(b" ");
                }
            }
        }

        RESPData::SimpleString(SimpleRESP {
            value: res.freeze(),
        })
    }
}

impl<'echo> Echo<'echo> {
    pub fn new(data: &'echo RESPData) -> Box<dyn Command + 'echo> {
        Box::new(Echo { data })
    }
}
