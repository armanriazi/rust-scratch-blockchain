

use serde::Serialize;

use super::*;
use crate::transaction::Put;

use crate::transaction::IO;
use std::collections::HashSet;



/// Collection of related blocks as the same as linked lists
/// In this program logic of POW algorithm have used.
#[derive(Debug,Serialize)]
pub struct Blockchain  {
    pub blocks: Vec<Block >,
    unspent_outputs: HashSet<Hash>,
}

impl  Default for Blockchain  {
    fn default() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
}
impl  Blockchain  {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    /// In the update_with_block() checking for (+)Overspending, (+)Double Spending, (-)Impersonate.
    /// Define BlockValidationError for violation of the rules POW
    /// Call input and output hash function  
    /// Retain unspent_outputs of the Blockchain
    pub fn blockchain_update_with_block(&mut self, block: Block ) -> Result<&Vec<Block>, CustomError> {
        let i = &self.blocks.len();

        if block.index != *i as u32 {
            return Err(CustomError::BlockValidation(
                BlockValidationError::MismatchedIndex,
            ));
            
        } else if !block::blockchain_check_difficulty(&block.hash(), block.difficulty) {
            return Err(CustomError::BlockValidation(
                BlockValidationError::InvalidHash,
            ));
        } else if *i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(CustomError::BlockValidation(
                    BlockValidationError::AchronologicalTimestamp,
                ));
            } else if block.prev_block_hash != prev_block.hash {
                return Err(CustomError::BlockValidation(
                    BlockValidationError::MismatchedPreviousHash,
                ));
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(CustomError::BlockValidation(
                    BlockValidationError::InvalidGenesisBlockFormat,
                ));
            }
        }

        if let Some((coinbase, option_transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(CustomError::BlockValidation(
                    BlockValidationError::InvalidCoinbaseTransaction,
                ));
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
          
            for transaction in option_transactions {  
                let input_hashes = transaction.returns_closure_io_hash(&IO::Input);
                let output_hashes = transaction.returns_closure_io_hash(&IO::Output);

                // info!("---------------------------\n");
                // info!("input_hashes {:?}\n",input_hashes());
                // info!("output_hashes {:?}\n",output_hashes());
                // info!("unspent_outputs {:?}\n",&self.unspent_outputs);
                // info!("block_spent {:?}\n",&block_spent);
                // info!("---------------------------\n");
                                             
                if !(&input_hashes() - &self.unspent_outputs).is_empty()
                    || !(&input_hashes() & &block_spent).is_empty()
                {
                    if *i==0 {                                
                        //info!("input_hashes - unspent_outputs={}\n",(!(&input_hashes() - &self.unspent_outputs).is_empty()).to_string());
                        //info!("input_hashes & block_spent={}\n",(!(&input_hashes() & &block_spent).is_empty()).to_string());
                    
                        return Err(CustomError::BlockValidation(
                            BlockValidationError::InvalidInput
                        ));
                    }
                }

                let input_value = transaction.returns_closure_io(&IO::Input);
                let output_value = transaction.returns_closure_io(&IO::Output);

                if &output_value() > &input_value() {
                    return Err(CustomError::BlockValidation(
                        BlockValidationError::InsufficientInputValue,
                    ));
                }

                let fee = &input_value() - &output_value();

                total_fee += fee;

                block_spent.extend(input_hashes());
                block_created.extend(output_hashes());
            }

            let coinbase_output_value = coinbase.returns_closure_io(&IO::Output);

            if coinbase_output_value() < total_fee {
                return Err(CustomError::BlockValidation(
                    BlockValidationError::InvalidCoinbaseTransactionFee,
                ));
            } else {
                let coinbase_output_hashes = coinbase.returns_closure_io_hash(&IO::Output);
                block_created.extend(coinbase_output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }
        println!("**BlOcKcHaIn SiGnAls:**\n{:?}\n", &block);
        let _ = &self.blocks.push(block);

        Ok(&self.blocks)
    }
}
