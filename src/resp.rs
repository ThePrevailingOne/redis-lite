use std::collections::VecDeque;

use bytes::Bytes;

pub const CLRF: &[u8; 2] = b"\r\n";
pub const CLRF_LEN: usize = 2;

pub struct SimpleRESP {
    pub value: Bytes,
}

pub struct AggregateRESP {
    pub children: VecDeque<RESPData>,
}

pub enum RESPData {
    SimpleString(SimpleRESP),
    SimpleError(SimpleRESP),
    Integer(SimpleRESP),
    Array(AggregateRESP),
    BulkString(AggregateRESP),
    NoType(SimpleRESP),
    Null,
}
