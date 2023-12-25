use bytes::BytesMut;

use crate::resp::*;

pub fn serialize_resp(data: RESPData, bytes: &mut BytesMut) {
    // TODO: populate for other data type
    match data {
        RESPData::SimpleString(s) => serialize_simple_string(s, bytes),
        RESPData::SimpleError(s) => serialize_simple_error(s, bytes),
        _ => {}
    };
}

fn serialize_simple_string(data: SimpleRESP, bytes: &mut BytesMut) {
    bytes.extend_from_slice(b"+");
    bytes.extend_from_slice(data.value.as_ref());
    bytes.extend_from_slice(CLRF);
}

fn serialize_simple_error(data: SimpleRESP, bytes: &mut BytesMut) {
    bytes.extend_from_slice(b"-");
    bytes.extend_from_slice(data.value.as_ref());
    bytes.extend_from_slice(CLRF);
}
