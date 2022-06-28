// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
//#![feature(doc_cfg)]

use std::{fmt::{ self, Debug, Formatter }, rc::Rc, cell::{RefCell, Ref, Cell}, borrow::Borrow};
use crate::transaction::OptionTransaction;
use super::*;


/// payload include transactions,difficulty,..
/// </br></br>
/// Order store of bytes: there are 2 types big-endian like 00 00 00 2a, little-endian like (our choice) 2a 00 00 00, u128 is in edian order, so because this material 16bytes of our hash will appear at the end of out hash's byte sector. 
/// </br></br>
/// nonce is just field for changes in block as an arbitary that hashed along with the data. so generating the correct hash for a block is like the puzzle , and the nonce is the key to that puzzle. the process of finding that key is called mining.
/// </br></br>
/// Impersonation: This can be solved by adding a "Signature" to outpus to verify they are being spent by their owner.(We can't assume that whoever sent us the trx over the network is also the person who created the trx. For now we'll kind of ignore solving this problem. we might come back to it when we go over smart contracts)
/// </br></br>
/// DoubleSpending: Make Sure that any one output is never used as an input more than once. This can be done by maintaining a pool of unspent outputs and rejecting any trx that tries to spend outputs that don't exist in the pool.
/// </br></br>
/// Inputs: unused outputs from prev TRXs, Outputs: new outouts That can be used in future TRXs.
/// </br></br>
/// OverSpending: Sum(inputs)>=Sum(Outputs). I can't input 5 coins and be able to output 7. (on other hand inputs have to be greater since must be enough fee in input section for paying to miner.)

pub struct Block<'a> {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub option_transactions: Rc<Cell<&'a Vec<OptionTransaction>>>,
    pub difficulty: u128, 
}



impl<'a> Debug for Block<'a> {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let y=self.option_transactions.take();
        write!(f, "Prev hash of {} the Block[{}]: {} at: {} trx.len: {} nonce: {}",
            &hex::encode(&self.prev_block_hash),
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            self.option_transactions.take().len(),
            &self.nonce,
        )
    }
}


impl<'a> Block<'a> {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash, option_transactions: Rc<Cell<&'a Vec<OptionTransaction>>>, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32], 
            prev_block_hash,
            nonce: 0,
            option_transactions,
            difficulty,
        }
}


/// 0xfff... lowest difficulty 
/// </br>
/// 0x000... => highest difficulty => taking more time=> more highest nonce=> the end of blockhash view see more zero so nonce 0 means end of of blockchash there isn'nt any zero
/// </br></br>
/// nonce is just field for changes in block as an arbitary that hashed along with the data. so generating the correct hash for a block is like the puzzle , and the nonce is the key to that puzzle. the process of finding that key is called mining.
/// </br></br>
/// mining sterategy: 
/// </br>
/// 1.Generate nonce 2.Hash bytes 3.Check hash against difficulty(Insufficant? Go Step1 and Sufficient Go Step 4) 4. Add block to chain 5. Submit to peers
    pub fn mine (&mut self) -> Result<Hash,CustomError>{
      
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

/// Concatenate together all the bytes
/// </br></br>
/// Generate unique data fingerprint: the hash
impl<'a> Hashable for Block<'a> {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.option_transactions.take()
                .iter()
                .flat_map(|transaction| transaction.puts.as_ref().unwrap().bytes())
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
