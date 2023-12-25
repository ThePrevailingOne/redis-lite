use std::collections::VecDeque;

use bytes::{Bytes, BytesMut};

pub const CLRF: &[u8; 2] = b"\r\n";
pub const CLRF_LEN: usize = 2;

#[derive(Debug)]
pub struct SimpleRESP {
    pub value: Bytes,
}

#[derive(Debug)]
pub struct AggregateRESP {
    pub children: VecDeque<RESPData>,
}

#[derive(Debug)]
pub enum RESPData {
    SimpleString(SimpleRESP),
    SimpleError(SimpleRESP),
    Integer(SimpleRESP),
    Array(AggregateRESP),
    BulkString(AggregateRESP),
    NoType(SimpleRESP),
    Null,
}

pub fn bytes_from_bulk_string(data: &RESPData) -> Bytes {
    let mut bytes = BytesMut::new();

    if let RESPData::BulkString(a) = data {
        for child in &a.children {
            if let RESPData::NoType(b) = child {
                bytes.extend(&b.value);
            }
        }
    }

    bytes.freeze()
}
