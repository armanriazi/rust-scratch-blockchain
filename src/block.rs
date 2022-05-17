// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
//#![feature(doc_cfg)]

use std::fmt::{ self, Debug, Formatter };


use super::*;


/// payload include transactions,difficulty,..
/// Order store of bytes: there are 2 types big-endian like 00 00 00 2a, little-endian like (our choice) 2a 00 00 00, u128 is in edian order, so because this material 16bytes of our hash will appear at the end of out hash's byte sector. 
/// nonce is just field for changes in block as an arbitary that hashed along with the data. so generating the correct hash for a block is like the puzzle , and the nonce is the key to that puzzle. the process of finding that key is called mining.

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128, 
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} trx.len: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}


impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash, transactions: Vec<Transaction>, difficulty: u128) -> Self {
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
/// 0xfff... lowest difficulty 
/// 0x000... => highest difficulty => taking more time=> more highest nonce=> the end of blockhash view see more zero so nonce 0 means end of of blockchash there isn'nt any zero
/// nonce is just field for changes in block as an arbitary that hashed along with the data. so generating the correct hash for a block is like the puzzle , and the nonce is the key to that puzzle. the process of finding that key is called mining.
/// mining sterategy: 1.Generate nonce 2.Hash bytes 3.Check hash against difficulty(Insufficant? Go Step1 and Sufficient Go Step 4) 4. Add block to chain 5. Submit to peers
    pub fn mine (&mut self) {
      
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();            
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

/// Concatenate together all the bytes
/// Generate unique data fingerprint: the hash
impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>()
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

/// Verify four things:
/// Actual Index, Block's hash fits stored difficulty value, Time is always increase, Actual previous block's hash
/// Difficulty: the most significant 16 bytes of the hash of a block must be less than before it is considered "valid"(if those bytes are interoreted as a single number instead of a serices of bytes.)
pub fn check_difficulty (hash: &Hash, difficulty: u128) -> bool {
    let result=difficulty_bytes_as_u128(&hash);
    difficulty > result
}
