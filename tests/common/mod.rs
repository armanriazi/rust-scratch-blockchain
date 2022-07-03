
use proptest::prelude::*;
// Credit: https://stackoverflow.com/a/44378174/2773837
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::HashSet;
use std::{
    borrow::Borrow,
    cell::{Cell, Ref, RefCell},
    fmt::{self, Debug, Formatter},
    os::unix::prelude::OsStrExt,
    rc::Rc,
};
use std::{ops::Deref};

extern crate serde;

use serde::{Deserialize, Serialize};

use crate::tests;


type Hash = Vec<u8>;
type Address = String;


pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
}



pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}



pub struct Blockchain<'a> {
    pub blocks: Vec<Block<'a>>,
    unspent_outputs: HashSet<Hash>,
}


pub struct Block<'a> {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: &'a mut Rc<Cell<&'a [Transaction<'a>]>>,
    pub difficulty: u128,
}


pub struct Transaction<'a> {
    pub inputs: Vec<Amount<'a>>,
    pub outputs: Vec<Amount<'a>>,
}



pub struct Amount<'a> {
    pub to_addr: &'a Address,
    pub amount: &'a u64,
}

pub enum IO {
    Input,
    Output,
}
pub enum IOH {
    Input,
    Output,
}


// impl Debug for Block<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Block").field("index", &self.index).field("timestamp", &self.timestamp).field("hash", &self.hash).field("prev_block_hash", &self.prev_block_hash).field("nonce", &self.nonce).field("transactions", &self.transactions).field("difficulty", &self.difficulty).finish()
//     }
// }

impl<'a> Default for Blockchain<'a> {
    fn default() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
}
impl<'a> Blockchain<'a> {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block<'a>) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if ! check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, option_transactions)) = block.transactions.take().split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in option_transactions {
                let input_hashes = transaction.returns_closure_io_hash(&IOH::Input);
                let output_hashes = transaction.returns_closure_io_hash(&IOH::Output);

                if !(&input_hashes() - &self.unspent_outputs).is_empty()
                    || !(&input_hashes() & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.returns_closure_io(&IO::Input);
                let output_value = transaction.returns_closure_io(&IO::Output);

                if &output_value() > &input_value() {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = &input_value() - &output_value();

                total_fee += fee;

                block_spent.extend(input_hashes());
                block_created.extend(output_hashes());
            }

            let coinbase_output_value = coinbase.returns_closure_io(&IO::Output);

            if coinbase_output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                let coinbase_output_hashes = coinbase.returns_closure_io_hash(&IOH::Output);
                block_created.extend(coinbase_output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }
        //info!("**Maked Blockchain:**\n{:?}\n", &block);
        self.blocks.push(block);

        Ok(())
    }
}


impl<'a> Block<'a> {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: &'a mut Rc<Cell<&'a [Transaction<'a>]>>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    pub fn mine(&mut self) -> Result<Hash, CustomError> {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();

            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;

                break;
            }
        }
        Ok(self.hash.clone())
    }
}


impl<'a> Hashable for Block<'a> {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .take()
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}
fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    let result = difficulty_bytes_as_u128(&hash);
    difficulty > result
}


//pub trait Put where Self: Sized {}
pub trait SuperTransaction {}

pub trait Put
where
    Self: SuperTransaction,
{
    fn returns_closure_io(&self, io: &IO) -> Box<(dyn Fn() -> u64 + '_)>;
    fn returns_closure_io_hash(&self, io: &IOH) -> Box<(dyn Fn() -> HashSet<Hash> + '_)>;
}

impl<'a> SuperTransaction for Transaction<'a> {}
//impl Put for SuperTransaction {}

impl<'a> Put for Transaction<'a> {
    fn returns_closure_io(&self, io: &IO) -> Box<(dyn Fn() -> u64 + '_)> {
        match io {
            IO::Input => Box::new(|| self.inputs.iter().map(|input| input.amount).sum()),
            IO::Output => Box::new(|| self.outputs.iter().map(|output| output.amount).sum()),
        }
    }

    fn returns_closure_io_hash(&self, io: &IOH) -> Box<(dyn Fn() -> HashSet<Hash> + '_)> {
        match io {
            IOH::Input => Box::new(|| {
                self.inputs
                    .iter()
                    .map(|input| input.hash())
                    .collect::<HashSet<Hash>>()
            }),
            IOH::Output => Box::new(|| {
                self.outputs
                    .iter()
                    .map(|output| output.hash())
                    .collect::<HashSet<Hash>>()
            }),
        }
    }
}

impl<'a> Transaction<'a> {

    pub fn new(inputs: Vec<Amount<'a>>, outputs: Vec<Amount<'a>>) -> Transaction<'a> {
        Self {
            inputs: inputs,
            outputs: outputs,
        }
    }

    pub fn is_coinbase(&self) -> bool {
        (&self.inputs).len() as u8 == 0
    }
}

impl<'a> Hashable for Amount<'a> {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.amount));

        bytes
    }
}

impl<'a> Hashable for Transaction<'a> {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes
    }
}

///sha-256 means generate 32 byte hash
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}


    fn now() -> u128 {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
    }


    fn u32_bytes(u: &u32) -> [u8; 4] {
        [
            (u >> 8 * 0x0) as u8,
            (u >> 8 * 0x1) as u8,
            (u >> 8 * 0x2) as u8,
            (u >> 8 * 0x3) as u8,
        ]
    }


    fn u64_bytes(u: &u64) -> [u8; 8] {
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


    fn u128_bytes(u: &u128) -> [u8; 16] {
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


    fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
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


 enum StringError {
    StringParse(std::string::ParseError),
    InvalidOption(String),
    Other,
}


 enum BlockainFactoryError {
    ZeroBlock,
    ZeroBlockchain,
    IsNullTransaction,
    Other,
}
/// Allow the use of "{:?}" format specifier

 enum CustomError {
    String(StringError),
    SerdeJson(serde_json::Error),
    IO(std::io::Error),
    BlockchainFactory(BlockainFactoryError),
    InvalidOption(String),
    Other,
}

// impl fmt::Display for CustomError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             CustomError::String(_) => write!(f, "String Error"),
//             CustomError::SerdeJson(ref cause) => write!(f, "SerdeJson Error: {}", cause),
//             CustomError::IO(ref cause) => write!(f, "IO Error: {}", cause),
//             CustomError::BlockchainFactory(_) => write!(f, "BlockchainFactory error!"),
//             CustomError::Other => write!(f, "Unknown error!"),
//             CustomError::InvalidOption(_) => write!(f, "Invalid Option!"),
//         }
//     }
// }

// impl From<json::de::Error> for Error {
//     fn from(err: toml::de::Error) -> Self {
//         Error::Toml(err)
//     }
// }

// impl std::error::Error for CustomError {
//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         match *self {
//             CustomError::String(_) => None,
//             CustomError::SerdeJson(ref cause) => Some(cause),
//             CustomError::IO(ref cause) => Some(cause),
//             CustomError::BlockchainFactory(_) => None,
//             CustomError::Other => None,
//             CustomError::InvalidOption(_) => None,
//         }
//     }
// }

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
