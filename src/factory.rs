use super::*;
use library_blockchain::transaction::{Amount, Transaction};

use std::cell::{Cell};
use std::rc::Rc;

#[allow(dead_code)]
#[allow(unused_mut)]
// #[deny(elided_lifetimes_in_paths)]
pub fn blockchain_factory<F>(
    mut blockchain: Blockchain,
    difficulty: u128,
    f: F,
) -> Result<(), CustomError>
where
    F: FnOnce() -> Result<serde_json::Value, CustomError>,
{
    let serde_values_transactions: serde_json::Value =
        serde_json::from_value(f().unwrap())?;//.unwrap();
    let blocks_val: serde_json::Value = serde_values_transactions["blocks"].clone();
    let mut prev_hash: Box<[u8]> = Box::default();
    blocks_val[0]
        .as_object()
        .unwrap()
        .into_iter()
        .enumerate()
        .for_each(|(_i, block)| {
            
            let mut maked_transactions_of_a_block: Vec<Transaction> = vec![];
            //info!("\nBlock {:?}\n",block);
            block
                .1
                .as_array()
                .unwrap()
                .into_iter()
                .enumerate()
                .for_each(|(_j, trxs)| {
                    let transactions = trxs.get("transactions").unwrap();
                    let obg_trx = transactions.as_array().unwrap();
                    let trx = obg_trx[0].as_object().unwrap();
                    let length = &trx.keys().len() + 1;
                    //info!("\n{:?}\n",trx);

                    for c in 1..length {
                        let trx_name = concat_string!("transaction", c.to_string());
                        let trx = (transactions[0].as_object().unwrap())
                            .get(&trx_name)
                            .unwrap();
                        let puts: Transaction = transaction_split(trx).unwrap();
                        //info!("\n{:?}\n",puts);
                        maked_transactions_of_a_block.push(puts);
                    }
                });

            let mut c: Cell<Vec<Transaction>> =
                    Cell::new(maked_transactions_of_a_block.to_vec());
            let mut r: Rc<Cell<Vec<Transaction>>> = Rc::new(c);
            let mut rc: &mut Rc<Cell<Vec<Transaction>>> = &mut r;

            if _i == 0 {
                let mut genesis_block = Block::new(0, now(), vec![0; 32], &mut Rc::clone(rc), difficulty);
                prev_hash = genesis_block.mine().unwrap().into_boxed_slice();
                //info!("**Mined_hash:**\n{:?}\n",prev_hash.clone());                
                update_blockchain_result(&mut blockchain, genesis_block,&_i);
                
            } else if _i > 0 {
                let mut maked_block: Block =
                    Block::new(_i as u32, now(), prev_hash.clone().to_vec(), &mut Rc::clone(rc), difficulty);
                prev_hash = maked_block.mine().unwrap().into_boxed_slice();
                //info!("**Mined_hash:**\n{:?}\n",prev_hash.clone());                
                update_blockchain_result(&mut blockchain, maked_block,&_i);                
            }
        });

    Ok(())
}

fn update_blockchain_result(blockchain: &mut Blockchain, block: Block,& i:&usize) {
    
    //std::process::exit(
        
        match blockchain.update_with_block(block) {
        Ok(_) => {
            info!("Success updated With the block {}.\n", 1+i.to_owned());
            //info!("**Maked_hash:**\n{:?}\n",&blockchain.blocks[i].prev_block_hash.clone());
            //0
        }
        Err(err) => {
            eprintln!("Did not update on the Blockchain-Error in the block {} : {err:?}.",1+i.to_owned());
            //1
        }
    }
    //);
}

fn transaction_split(trx: &serde_json::Value) -> Result<Transaction, CustomError> {
    let mut trx_inputs_model_vec: Vec<Amount> = vec![];
    let new_option_transaction: Transaction;

    if trx.is_null() {
        return Err(CustomError::BlockchainFactory(BlockainFactoryError::Other));
    }
    if trx[0].as_object().unwrap().is_empty() || trx[0].as_object().unwrap().len() < 1usize {
        return Err(CustomError::BlockchainFactory(
            BlockainFactoryError::IsNullTransaction,
        ));
    }
    let trx_inputs = (trx[0].as_object().unwrap()).get("inputs").unwrap();

    if !(trx_inputs.is_null()) && !(trx_inputs.as_array().is_none()) {
        let trx_inputs_vec = trx_inputs.as_array().unwrap();

        for item_internal_inputs in trx_inputs_vec {
            let trx_inputs_model ;

            if !item_internal_inputs.is_null() {
                if !((item_internal_inputs["value"].is_null()
                    && item_internal_inputs["to_addr"].is_null())
                    || (item_internal_inputs["to_addr"].as_str().unwrap().is_empty()
                        && item_internal_inputs["value"].as_str().unwrap().is_empty()))
                {
                    trx_inputs_model = Amount {
                        to_addr: item_internal_inputs["to_addr"].as_str().unwrap().to_owned(),
                        amount: item_internal_inputs["value"]
                            .as_str()
                            .unwrap()
                            .parse::<u64>()
                            .unwrap(),
                    };
                    trx_inputs_model_vec.push(trx_inputs_model);
                }
            }
        }
    }

    let mut trx_outputs_model_vec: Vec<Amount> = vec![];
    let trx_outputs = (trx[0].as_object().unwrap()).get("outputs").unwrap();

    if !(trx_outputs.is_null()) && !(trx_outputs.as_array().is_none()) {
        let trx_outputs_vec = trx_outputs.as_array().unwrap();

        for item_internal_outputs in trx_outputs_vec {
            let trx_outputs_model ;

            if !item_internal_outputs.is_null() {
                if !((item_internal_outputs["value"].is_null()
                    && item_internal_outputs["to_addr"].is_null())
                    || item_internal_outputs["value"].as_str().unwrap().is_empty()
                        && item_internal_outputs["to_addr"]
                            .as_str()
                            .unwrap()
                            .is_empty())
                {
                    trx_outputs_model = Amount {
                        to_addr: item_internal_outputs["to_addr"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                        amount: item_internal_outputs["value"]
                            .as_str()
                            .unwrap()
                            .parse::<u64>()
                            .unwrap(),
                    };
                    trx_outputs_model_vec.push(trx_outputs_model);
                }
            }
        }
        new_option_transaction = Transaction::new(trx_inputs_model_vec, trx_outputs_model_vec);

        return Ok(new_option_transaction);
    }
    return Err(CustomError::BlockchainFactory(BlockainFactoryError::Other));
}
