use std::collections::VecDeque;

use bytes::Bytes;

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
