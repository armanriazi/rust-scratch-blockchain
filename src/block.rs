// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
//#![feature(doc_cfg)]

use serde::{Serialize};

use super::*;
use crate::transaction::Transaction;
use std::{
    cell::{Cell},
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

/// Payload include transactions, difficulty,..
/// </br></br>
/// Order store of bytes: there are 2 types big-endian like 00 00 00 2a, little-endian like (our choice) 2a 00 00 00, u128 is in edian order, so because this material 16bytes of our hash will appear at the end of out hash's byte sector.
/// </br></br>
/// nonce is just field for changes in block as an arbitary that hashed along with the data. so generating the correct hash for a block is like the puzzle , and the nonce is the key to that puzzle. the process of finding that key is called mining.
/// </br></br>
/// </br></br>
/// Overspending: where did the money come from?  inputs must be >= sum of values of generated outputs.
/// </br></br>
/// OverSpending: Sum(inputs)>=Sum(Outputs). I can't input 5 coins and be able to output 7. (on other hand inputs have to be greater since must be enough fee in input section for paying to miner.)
/// </br></br>
/// Impersonation: this can be solved by adding a "Signature" to outpus to verify they are being spent by their owner.(We can't assume that whoever sent us the trx over the network is also the person who created the trx. For now we'll kind of ignore solving this problem. we might come back to it when we go over smart contracts).
/// </br></br>
/// Impersonation: who owns the money and who is sending it?  Solved by adding signature and smart contract(not cover in this example).
/// </br></br>
/// DoubleSpending: make sure that anyone output is never used as an input more than once. This can be done by maintaining a pool of unspent outputs and rejecting any trx that tries to spend outputs that don't exist in the pool.
/// </br></br>
/// Double Spending: is the money avaliable? any one output is never used as an input more than once.
/// </br></br>
/// </br></br>
/// Inputs: unused outputs from prev TRXs, Outputs: new outouts That can be used in future TRXs.

#[derive(Serialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions:  Vec<Transaction> ,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} trx, nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
       )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: &mut Rc<Cell<Vec<Transaction>>>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions: transactions.take().to_vec(),
            difficulty,
        }
    }

    /// 0xfff... lowest difficulty
    /// </br>
    /// 0x000... => highest difficulty => taking more time=> more highest nonce=> the end of blockhash see more zero so nonce 0 means end of of blockchash there isn'nt any zero at the end of blockhash.
    /// </br></br>
    /// nonce is just field for changes in block as an arbitary that hashed along with the data. so generating the correct hash for a block is like the puzzle , and the nonce is the key to that puzzle. the process of finding that key is called mining.
    /// </br></br>
    /// mining sterategy:
    /// </br>
    /// 1.Generate nonce 2.Hash bytes 3.Check hash against difficulty(Insufficant? Go Step1 and Sufficient Go Step 4) 4. Add block to chain 5. Submit to peers
    pub fn blockchain_mine(&mut self) -> Result<Hash, CustomError> {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();

            if blockchain_check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return Ok(self.hash.clone())
             
            }
        }
        return Err(CustomError::BlockValidation(BlockValidationError::NonceAttemptFailed));
      
    }
}

/// Concatenate together all the bytes
/// </br></br>
/// Generate unique data fingerprint: the hash
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&lib_block_u32_bytes(&self.index));
        bytes.extend(&lib_block_u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&lib_block_u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions                
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&lib_block_u128_bytes(&self.difficulty));

        bytes
    }
}

/// Verify four things:
/// Actual Index, Block's hash fits stored difficulty value, Time is always increase, Actual previous block's hash
/// Difficulty: the most significant 16 bytes of the hash of a block must be less than before it is considered "valid"(if those bytes are interoreted as a single number instead of a serices of bytes.)
pub fn blockchain_check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    let result = lib_block_difficulty_bytes_as_u128(&hash);
    difficulty > result
}
