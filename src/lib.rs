#![recursion_limit="5000000"]

mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::{hashable::Hashable, blockchain::Blockchain};
mod blockchain;
pub mod transaction;
pub use crate::transaction::Transaction;

type Hash = Vec<u8>;
type Address = String;

use std::fmt;
// Credit: https://stackoverflow.com/a/44378174/2773837
use std::time::{ SystemTime, UNIX_EPOCH };

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
    ;

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

pub fn u128_bytes (u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,

        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,

        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

pub fn difficulty_bytes_as_u128 (v: &Vec<u8>) -> u128 {
   ((v[31] as u128) << 0xf * 8) |
    ((v[30] as u128) << 0xe * 8) |
    ((v[29] as u128) << 0xd * 8) |
    ((v[28] as u128) << 0xc * 8) |
    ((v[27] as u128) << 0xb * 8) |
    ((v[26] as u128) << 0xa * 8) |
    ((v[25] as u128) << 0x9 * 8) |
    ((v[24] as u128) << 0x8 * 8) |
    ((v[23] as u128) << 0x7 * 8) |
    ((v[22] as u128) << 0x6 * 8) |
    ((v[21] as u128) << 0x5 * 8) |
    ((v[20] as u128) << 0x4 * 8) |
    ((v[19] as u128) << 0x3 * 8) |
    ((v[18] as u128) << 0x2 * 8) |
    ((v[17] as u128) << 0x1 * 8) |
    ((v[16] as u128) << 0x0 * 8)
}


/// Allow the use of "{:?}" format specifier
#[derive(Debug)] 
pub enum CustomError {
    StringParse(std::string::ParseError),
    SerdeJson(serde_json::Error),
    IO(std::io::Error),
    BlockchainFactory,
    Other,
}


// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::StringParse(ref cause) => write!(f, "StringParse Error: {}", cause),
            CustomError::SerdeJson(ref cause) => write!(f, "SerdeJson Error: {}", cause),
            CustomError::IO(ref cause) => write!(f, "IO Error: {}", cause),
            CustomError::BlockchainFactory => write!(f, "BlockchainFactory error!"),
            CustomError::Other => write!(f, "Unknown error!"),
        }
    }
}
impl std::error::Error for CustomError{
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CustomError::StringParse(ref cause) => Some(cause),
            CustomError::SerdeJson(ref cause) => Some(cause),
            CustomError::IO(ref cause) => Some(cause),
            CustomError::BlockchainFactory => None,
            CustomError::Other => None,
        }
    }


}
impl From<std::string::ParseError> for CustomError {
    fn from(cause: std::string::ParseError) -> CustomError {
        CustomError::StringParse(cause)
    }
}
impl From<serde_json::Error> for CustomError {
    fn from(cause: serde_json::Error) -> CustomError {
        CustomError::SerdeJson(cause)
    }
}    
impl From<std::io::Error> for CustomError {
        fn from(cause: std::io::Error) -> CustomError {
            CustomError::IO(cause)
        }    
}

