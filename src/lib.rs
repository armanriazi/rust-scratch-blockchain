// #![recursion_limit="5000000"]

#![no_main] 
pub mod block;
pub mod blockchain_executive;
pub use crate::block::Block;
pub mod hashable;
pub use crate::{blockchain::Blockchain, hashable::Hashable};
pub mod blockchain;
pub mod transaction;
pub mod factory;
type Hash = Vec<u8>;
type Address = String;

use std::fmt::{self, Formatter};
use std::os::unix::prelude::MetadataExt;
// Credit: https://stackoverflow.com/a/44378174/2773837
use std::time::{SystemTime, UNIX_EPOCH};




pub fn lib_block_now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn lib_block_u32_bytes(u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn lib_block_u64_bytes(u: &u64) -> [u8; 8] {
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

pub fn lib_block_u128_bytes(u: &u128) -> [u8; 16] {
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

pub fn lib_block_difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 0xf * 8)
        | ((v[30] as u128) << 0xe * 8)
        | ((v[29] as u128) << 0xd * 8)
        | ((v[28] as u128) << 0xc * 8)
        | ((v[27] as u128) << 0xb * 8)
        | ((v[26] as u128) << 0xa * 8)
        | ((v[25] as u128) << 0x9 * 8)
        | ((v[24] as u128) << 0x8 * 8)
        | ((v[23] as u128) << 0x7 * 8)
        | ((v[22] as u128) << 0x6 * 8)
        | ((v[21] as u128) << 0x5 * 8)
        | ((v[20] as u128) << 0x4 * 8)
        | ((v[19] as u128) << 0x3 * 8)
        | ((v[18] as u128) << 0x2 * 8)
        | ((v[17] as u128) << 0x1 * 8)
        | ((v[16] as u128) << 0x0 * 8)
}

/// In the stable version rust that we used is concat macro.
/// </br>
/// 🔬 For a nightly-only experimental API. (slice_concat_trait #27747)
/// https://doc.rust-lang.org/std/slice/trait.Concat.html#
/// Another Solution </br>
/// #[macro_use(concat_string)]</br>
/// ```
/// extern crate concat_string;
/// pub fn blockchain_concat_two_str(str:String,num:String)-> String {    
///     let output=concat_string!(str,num);    
///     output
/// }
/// ```
pub fn lib_block_concat_two_string(str1:String,str2:String) -> String{
    let mut str =&mut str1.to_string().to_owned();
    str.push_str(&str2);
    let output:String =str.chars().filter(|c| !c.is_whitespace()).collect();
    output
}

#[derive(Debug)]
pub enum StringError {
    StringParse(std::string::ParseError),
    InvalidOption(String),
    Other,
}
#[derive(Debug)]
pub enum BlockainFactoryError {
    ZeroBlock,
    ZeroBlockchain,
    IsNullTransaction,
    Other,
}
#[derive(Debug)]
pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
    InvalidCoinbaseTransactionFee,
    NonceAttemptFailed
}
/// Allow the use of "{:?}" format specifier
#[derive(Debug)]
pub enum CustomError {
    String(StringError),
    SerdeJson(serde_json::Error),
    IO(std::io::Error),
    BlockchainFactory(BlockainFactoryError),
    BlockValidation(BlockValidationError),    
    InvalidOption(String),
    Other,
}

impl fmt::Display for crate::BlockainFactoryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
                BlockainFactoryError::IsNullTransaction => write!(f, "BlockainFactory:IsNullTransaction Error"),
                BlockainFactoryError::ZeroBlock => write!(f, "BlockainFactory:ZeroBlock Error"),
                BlockainFactoryError::ZeroBlockchain => write!(f, "BlockainFactory:ZeroBlockchain Error"),
                BlockainFactoryError::Other => write!(f, "BlockainFactory:Unknown Error"),                
            }
        }
}

impl fmt::Display for crate::BlockValidationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
                BlockValidationError::AchronologicalTimestamp => write!(f, "BlockValidation:AchronologicalTimestamp Error"),
                BlockValidationError::InsufficientInputValue => write!(f, "BlockValidation:InsufficientInputValue Error"),
                BlockValidationError::InvalidCoinbaseTransaction => write!(f, "BlockValidation:InvalidCoinbaseTransaction Error"),
                BlockValidationError::InvalidCoinbaseTransactionFee => write!(f, "BlockValidation:InvalidCoinbaseTransactionFee Error"),
                BlockValidationError::InvalidGenesisBlockFormat => write!(f, "BlockValidation:InvalidGenesisBlockFormat Error"),
                BlockValidationError::InvalidHash => write!(f, "BlockValidation:InvalidHash Error"),
                BlockValidationError::InvalidInput=> write!(f, "BlockValidation:InvalidInput Error"),
                BlockValidationError::MismatchedIndex=> write!(f, "BlockValidation:MismatchedIndex Error"),
                BlockValidationError::MismatchedPreviousHash=> write!(f, "BlockValidation:MismatchedPreviousHash Error"),
                BlockValidationError::NonceAttemptFailed=> write!(f, "BlockValidation:NonceAttemptFailed Error"),                
            }
        }
}
// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::String(_) => write!(f, "\nString Error"),
            CustomError::SerdeJson(ref cause) => write!(f, "\nSerdeJson Error: {}", cause),
            CustomError::IO(ref cause) => write!(f, "\nIO Error: {}", cause),
            CustomError::BlockchainFactory(ref cause) => write!(f, "\nBlockchainFactory Error: {}", cause),
            CustomError::BlockValidation(ref cause) => write!(f, "\nBlockValidation Error: {}", cause),
            CustomError::Other => write!(f, "\nUnknown error!"),
            CustomError::InvalidOption(_) => write!(f, "\nInvalid Option!"),
        }
    }
}

// impl From<json::de::Error> for Error {
//     fn from(err: toml::de::Error) -> Self {
//         Error::Toml(err)
//     }
// }

impl std::error::Error for CustomError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CustomError::String(_) => None,
            CustomError::SerdeJson(ref cause) => Some(cause),
            CustomError::IO(ref cause) => Some(cause),
            CustomError::BlockchainFactory(_) => None,
            CustomError::BlockValidation(_) => None,
            CustomError::Other => None,
            CustomError::InvalidOption(_) => None,
        }
    }
}

impl From<std::string::ParseError> for StringError {
    fn from(cause: std::string::ParseError) -> StringError {
        StringError::InvalidOption(cause.to_string())
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
impl From<BlockValidationError> for CustomError {
    fn from(cause: BlockValidationError) -> CustomError {
        CustomError::BlockValidation(cause)
    }
}
impl From<BlockainFactoryError> for CustomError {
    fn from(cause: BlockainFactoryError) -> CustomError {
        CustomError::BlockchainFactory(cause)
    }
}
