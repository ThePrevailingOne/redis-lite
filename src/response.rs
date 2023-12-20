use bytes::{Bytes, BytesMut};

use crate::resp::*;

pub fn serialize_resp(data: RESPData, bytes: &mut BytesMut) {
    // TODO: populate for other data type
    serialize_simple_string(data, bytes);
}

pub fn serialize_simple_string(data: RESPData, bytes: &mut BytesMut) {
    match data {
        RESPData::SimpleString(s) => {
            bytes.extend_from_slice(b"+");
            bytes.extend_from_slice(&s.value.as_ref());
            bytes.extend_from_slice(CLRF);
        }
        _ => {}
    };
}
