use core::fmt;
use std::error::Error;

use bytes::Bytes;

use crate::resp::{RESPData, SimpleRESP};

#[derive(Debug)]
pub struct Err {
    message: String,
}

impl Error for Err {}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom Error: {}", self.message)
    }
}

impl Err {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn to_resp(&self) -> RESPData {
        RESPData::SimpleError(SimpleRESP {
            value: Bytes::copy_from_slice(self.message.as_bytes()),
        })
    }
}
