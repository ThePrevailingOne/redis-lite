use crate::resp::*;
use std::collections::VecDeque;

use bytes::Bytes;
use log::info;

pub fn parse_request(buffer: &[u8]) {
    if buffer.is_empty() {
        log::error!("Buffer is empty!");
        return;
    }

    construct_data(buffer);
}

pub fn construct_data(mut buffer: &[u8]) -> (&[u8], RESPData) {
    // match first byte to data type

    if buffer.is_empty() {
        log::error!("Can't construct data; buffer is empty!");
    }

    let first_b = buffer.first().unwrap();
    let clrf_pos = buffer.windows(2).position(|seq| seq == CLRF).unwrap();
    let content = &buffer[1..clrf_pos];
    let nt_content = &buffer[..clrf_pos];
    buffer = &buffer[clrf_pos + CLRF_LEN..];

    let data = match first_b {
        b'+' => construct_simple_string(content),
        b'-' => construct_simple_error(content),
        b':' => construct_integer(content),
        b'$' => {
            let (b, d) = construct_bulk_string(1, buffer);
            buffer = b;
            d
        }
        b'*' => {
            let s = std::str::from_utf8(content).unwrap();
            let i = usize::from_str_radix(s, 10).unwrap();
            let (b, d) = construct_array(i, buffer);
            buffer = b;
            d
        }
        b'_' => construct_null(),
        _ => construct_no_type(nt_content),
    };
    (buffer, data)
}

pub fn construct_simple_string(content: &[u8]) -> RESPData {
    info!(
        "Constructing SimpleString: {:?}",
        std::str::from_utf8(content)
    );
    RESPData::SimpleString(SimpleRESP {
        value: Bytes::copy_from_slice(content),
    })
}

pub fn construct_simple_error(content: &[u8]) -> RESPData {
    info!(
        "Constructing SimpleError: {:?}",
        std::str::from_utf8(content)
    );
    RESPData::SimpleError(SimpleRESP {
        value: Bytes::copy_from_slice(content),
    })
}

pub fn construct_integer(content: &[u8]) -> RESPData {
    info!("Constructing Integer: {:?}", std::str::from_utf8(content));
    RESPData::Integer(SimpleRESP {
        value: Bytes::copy_from_slice(content),
    })
}

pub fn construct_array(children_count: usize, buffer: &[u8]) -> (&[u8], RESPData) {
    info!("Constructing Array: {}", children_count);
    let mut data = RESPData::Array(AggregateRESP {
        children: VecDeque::new(),
    });

    let mut b: &[u8] = buffer;
    if let RESPData::Array(a) = &mut data {
        b = populate_children(children_count, buffer, &mut a.children);
    }

    (b, data)
}

pub fn construct_bulk_string(children_count: usize, buffer: &[u8]) -> (&[u8], RESPData) {
    info!("Constructing BulkString: {}", children_count);
    let mut data = RESPData::BulkString(AggregateRESP {
        children: VecDeque::new(),
    });

    let mut b: &[u8] = buffer;
    if let RESPData::BulkString(a) = &mut data {
        b = populate_children(children_count, buffer, &mut a.children);
    }

    (b, data)
}

pub fn construct_no_type(content: &[u8]) -> RESPData {
    info!("Constructing NoType: {:?}", std::str::from_utf8(content));
    RESPData::NoType(SimpleRESP {
        value: Bytes::copy_from_slice(content),
    })
}

pub fn construct_null() -> RESPData {
    return RESPData::Null;
}

fn populate_children<'a>(
    children_count: usize,
    buffer: &'a [u8],
    list: &mut VecDeque<RESPData>,
) -> &'a [u8] {
    let mut buf = buffer;
    for _ in 0..children_count {
        let (b, child) = construct_data(buf);
        buf = b;
        list.push_back(child);
    }

    buf
}
