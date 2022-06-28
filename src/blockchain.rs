use crate::transaction::OptionTransaction;
use crate::transaction::Put;
use crate::transaction::IO as IO;
use crate::transaction::IOH as IOH;
use super::*;
use std::collections::HashSet;

#[derive(Debug)]
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

/// On update_with_block() we check (+)Overspending, (+)Double Spending, (-)Impersonate
pub struct Blockchain<'a> {
    pub blocks: Vec<Block<'a>>,
    unspent_outputs: HashSet<Hash>,
    
}

impl<'a> Default for Blockchain<'a>{
    
    fn default () -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
}
impl<'a> Blockchain<'a> {    
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
    
    pub fn update_with_block (&mut self, block: Block<'a>) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
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

        if let Some((coinbase, option_transactions)) =  block.option_transactions.take().split_first(){
            if ! coinbase.puts.as_ref().unwrap().is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
            
            
            for transaction in option_transactions {
  
                let input_hashes = transaction.puts.as_ref().unwrap().returns_closure_io_hash(&IOH::Input);
                let output_hashes = transaction.puts.as_ref().unwrap().returns_closure_io_hash(&IOH::Output);
                
                if
                    !(&input_hashes() - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes() & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value= transaction.puts.as_ref().unwrap().returns_closure_io(&IO::Input);
                let output_value = transaction.puts.as_ref().unwrap().returns_closure_io(&IO::Output);


                if &output_value()>&input_value() {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = &input_value()-&output_value();

                total_fee += fee;
                
                block_spent.extend(input_hashes());
                block_created.extend(output_hashes());
            }

            let coinbase_output_value = coinbase.puts.as_ref().unwrap().returns_closure_io(&IO::Output);
                    

            if coinbase_output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                let coinbase_output_hashes=coinbase.puts.as_ref().unwrap().returns_closure_io_hash(&IOH::Output);
                block_created.extend(coinbase_output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }
        println!("**Maked Blockchain:**\n{:?}\n",&block);
        self.blocks.push(block);
        
        Ok(())
    }
}
