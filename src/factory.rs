
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;

use super::*;
use library_blockchain::transaction::{Value as ModelValue, OptionTransaction};
use library_blockchain::{*};
use serde_json::Value;

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn blockchain_factory<F>(difficulty:u128, f : F) -> Result<(),CustomError>
    where   
    F: FnOnce()->  Result<serde_json::Value,CustomError>    
{
    let serde_values_transactions:serde_json::Value= serde_json::from_value(f().unwrap()).unwrap();
    let blocks_val:serde_json::Value=serde_values_transactions["blocks"].clone();   
    let mut blockchain=Blockchain::new();


    blocks_val[0].as_object().unwrap().into_iter().enumerate().for_each(|(i, block)| {

        //println!("\nBlock {:?}\n",block);
        block.1.as_array().unwrap().into_iter().enumerate().for_each(|(j, trxs)| {
        
        let transactions=trxs.get("transactions").unwrap();        
        let obg_trx=transactions.as_array().unwrap();                             
        let trx=obg_trx[0].as_object().unwrap();        
        let length=trx.keys().len()+1;   
        
        let maked_transaction:Vec<OptionTransaction>= fetch_raw_block_transactions(trx,length).unwrap();            
        if maked_transaction.len()==0 {
            return Err(CustomError::BlockchainFactory)
        }       
    
            // if i==1{
            //     let mut genesis_block = Block::new(0, now(),vec![0; 32], maked_transaction, difficulty);    
            //     prev_hash=genesis_block.mine().unwrap().into_boxed_slice();                
            //     let _=&blockchain.update_with_block(genesis_block);                
            // }
            // else if i >1{
            //     let mut maked_block:Block = Block::new(i as u32, now(), prev_hash.to_vec(), maked_transaction, difficulty);                                     
            //     prev_hash=maked_block.mine().unwrap().into_boxed_slice();
            //     let _=&blockchain.update_with_block(maked_block);
            //     //println!("**maked_hash:**\n{:?}\n",&blockchain.blocks[i].prev_block_hash.clone());                
            // }     
      });   
   });
                                         
    Ok(()) 
}          
 

fn transaction_split( trx:&serde_json::Value) -> Result<OptionTransaction,CustomError>{

    let mut trx_inputs_model_vec :Vec<ModelValue> = vec![];  
    let mut new_transaction:OptionTransaction;

    if trx.is_null(){        
        return Err(CustomError::BlockchainFactory)
    }
    if trx[0].as_object().unwrap().is_empty() || trx[0].as_object().unwrap().len()<1usize{
        return Err(CustomError::BlockchainFactory)
    }
        let trx_inputs=(trx[0].as_object().unwrap()).get("inputs").unwrap();                
                        
        if !(trx_inputs.is_null()) && !(trx_inputs.as_array().is_none()){                    
            let trx_inputs_vec=trx_inputs.as_array().unwrap();                    
            

            for item_internal_inputs in trx_inputs_vec {                                                              
                let mut trx_inputs_model:ModelValue=ModelValue{
                    to_addr: String::from(""),
                    value: 0,
                };
                
                if !item_internal_inputs.is_null(){
                
                    if !(item_internal_inputs["value"].is_null() && item_internal_inputs["to_addr"].is_null()){                      
                            trx_inputs_model= ModelValue{
                                to_addr:item_internal_inputs["to_addr"].as_str().unwrap().to_owned(),
                                value:item_internal_inputs["value"].as_str().unwrap().parse::<u64>().unwrap()
                            };
                            trx_inputs_model_vec.push(trx_inputs_model);                                       
                    }
                    
                }
            }                    
        }
                        
        let mut trx_outputs_model_vec :Vec<ModelValue> = vec![];  
        let trx_outputs=(trx[0].as_object().unwrap()).get("outputs").unwrap();                
                        
        if !(trx_outputs.is_null()) && !(trx_outputs.as_array().is_none()){                    
            let trx_outputs_vec=trx_outputs.as_array().unwrap();                    
            

            for item_internal_outputs in trx_outputs_vec {                                                              
                let mut trx_outputs_model:ModelValue=ModelValue{
                    to_addr: String::from(""),
                    value: 0,
                };
                
                if !item_internal_outputs.is_null(){
                
                    if !(item_internal_outputs["value"].is_null() && item_internal_outputs["to_addr"].is_null()){                      
                            trx_outputs_model= ModelValue{
                                to_addr:item_internal_outputs["to_addr"].as_str().unwrap().to_owned(),
                                value:item_internal_outputs["value"].as_str().unwrap().parse::<u64>().unwrap()
                            };
                            trx_outputs_model_vec.push(trx_outputs_model);                                        
                        }
                    
                    }
                }                                 
        new_transaction= Transaction::new(trx_inputs_model_vec,trx_outputs_model_vec);
        
        return Ok(new_transaction);                   
        }                       
        return Err(CustomError::BlockchainFactory) ;      
}

fn fetch_raw_block_transactions(transctions_map:Vec<serde_json::Value>,values_transactions_found_len:usize) -> Result<Vec<OptionTransaction>,CustomError>{
   
    for c in 1..length{
        let trx_name=concat_string!("transaction",c.to_string());
        let trx=(obg_trx[0].as_object().unwrap()).get(&trx_name).unwrap();
        println!("\n{:?}\n",trx_name);
        println!("\n{:?}\n",trx);

        println!("\n--------------------------------------\n");
    }
    // let mut maked_transaction:Vec<OptionTransaction> = vec![];  
    // println!("**j:**{:?}\n",&values_transactions_found_len);
    // for j in 1..values_transactions_found_len+1{       
    //     let transaction_str=concat_string!("transaction",j.to_string());
    //     let transaction_map0=&transctions_map[0]["transactions"].clone();
    //     let transaction_map1=&(transaction_map0[0].as_object().unwrap()).get_key_value(&transaction_str).unwrap();               
    //     println!("**maked transactions:**\n{:?}\n\n",transaction_split(&transaction_map1.1).unwrap());      
    //     let _=&maked_transaction.push(transaction_split(&transaction_map1.1).unwrap());            
    // } 
    Ok(maked_transaction)
}

