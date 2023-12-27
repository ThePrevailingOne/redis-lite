use std::collections::HashMap;

use bytes::Bytes;

pub enum Frame {
    String(String),
    Integer(i32),
}

pub struct Memory {
    kv_table: HashMap<Bytes, Frame>,
}

impl Frame {
    pub fn string_frame(bytes: &Bytes) -> Self {
        let s: &[u8] = bytes.as_ref();
        Frame::String(String::from_utf8_lossy(s).to_string())
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            kv_table: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: Bytes, frame: Frame) {
        self.kv_table.insert(key, frame);
    }

    pub fn get(&self, key: Bytes) -> &Frame {
        self.kv_table.get(&key).unwrap()
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

        memory
            .kv_table
            .insert(Bytes::from_static("test".as_bytes()), frame);

        assert!(matches!(
            memory.get(Bytes::from_static("test".as_bytes())),
            Frame::String(_)
        ));

        if let Frame::String(s) = memory.get(Bytes::from_static("test".as_bytes())) {
            assert_eq!(s, &String::from("Hello World!"));
        }
    }
}
