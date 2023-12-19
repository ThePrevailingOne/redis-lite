use std::collections::HashMap;

use bytes::Bytes;

enum Frame {
    String(String),
    Integer(i32),
}

struct Memory<'mem> {
    kv_table: HashMap<&'mem str, Frame>,
}

impl Frame {
    pub fn string_frame(bytes: &Bytes) -> Self {
        let s: &[u8] = bytes.as_ref();
        Frame::String(String::from_utf8_lossy(s).to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Bytes, Frame, Memory};

    #[test]
    fn test_string_frame() {
        let bytes = Bytes::from_static(b"Hello World!");
        let frame = Frame::string_frame(&bytes);

        assert!(matches!(frame, Frame::String(_)));

        if let Frame::String(s) = frame {
            assert_eq!(s, String::from("Hello World!"));
        }
    }

    #[test]
    fn test_memory() {
        let mut memory = Memory {
            kv_table: HashMap::new(),
        };

        let bytes = Bytes::from_static(b"Hello World!");
        let frame = Frame::string_frame(&bytes);

        memory.kv_table.insert("test", frame);

        assert!(matches!(
            memory.kv_table.get("test").unwrap(),
            Frame::String(_)
        ));

        if let Frame::String(s) = memory.kv_table.get("test").unwrap() {
            assert_eq!(s, &String::from("Hello World!"));
        }
    }
}
