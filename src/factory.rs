
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
    //let blocks_val_0=blocks_val[0].as_object().unwrap(); 
   // let len_blocks_map=blocks_val_0.len();       // Or :blocks_val_0.into_iter().count();
    //let values_blocks_found_len=len_blocks_map+1;   
    // if blocks_val.to_string().find("block")==None || values_blocks_found_len< 1usize{
    //     return Err(CustomError::BlockchainFactory)
    // }
    // //let values_blocks_found_len=blocks_val.to_string().find("block").unwrap();
    // let mut transactions:Vec<OptionTransaction> = vec![];  
    // let mut values_transactions_found:Vec<String>=vec![];
    // let mut prev_hash:Box<[u8]> =Box::default();
    // //---
    // println!("**Len block:**\n{:?}\n",values_blocks_found_len-1);    
        
        //println!("**blocks_map:**\n{:?}\n",i);
        //let block=blocks_map.get_key_value(&block_str).unwrap();                
        //if(blocks_map.get(&block_str).contains(&block_str).to_string()==Some(&block_str){

            blocks_val[0].as_object().unwrap().into_iter().enumerate().for_each(|(i, block)| {
 
            //let transactions_map:Vec<serde_json::Value>=block[i];    
            //let transactions_map=block[i].as_array().unwrap().get(0).clone().unwrap();
            //println!("{:?}\n",block);
            
            block.1.as_array().unwrap().into_iter().enumerate().for_each(|(j, trxs)| {
             
             let transactions=trxs.get("transactions").unwrap();
             let obg_trx=transactions.as_array().unwrap();             
             let yy=(obg_trx[0].as_object().unwrap()).get("transaction1").unwrap();    
             //obg_trx.into_iter().enumerate().for_each(|(j, trx)| {
             println!("\n-------------{:?}\n",yy);
             //});
             //  qq.to_owned().get("transaction1").unwrap().as_array().unwrap().into_iter().enumerate().for_each(|(j, trx)| {
            //     println!("\n-------------{:?}\n",trx);
            //  });

            
                     
                //trxs.as_object().unwrap().into_iter().enumerate().for_each(|(k, trx)| {
                    //println!("{:?}\n",transactions);
                
            });
            // let values_transactions_found_len=(temp_map_for_getting_len[0].as_object().unwrap()).len();                      
            
            // let maked_transaction:Vec<OptionTransaction>= fetch_raw_block_transactions(transactions_map,values_transactions_found_len).unwrap();            
            // if maked_transaction.len()==0 {
            //     return Err(CustomError::BlockchainFactory)
            // }       
    
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
    
    let mut maked_transaction:Vec<OptionTransaction> = vec![];  
    println!("**j:**{:?}\n",&values_transactions_found_len);
    for j in 1..values_transactions_found_len+1{       
        let transaction_str=concat_string!("transaction",j.to_string());
        let transaction_map0=&transctions_map[0]["transactions"].clone();
        let transaction_map1=&(transaction_map0[0].as_object().unwrap()).get_key_value(&transaction_str).unwrap();               
        println!("**maked transactions:**\n{:?}\n\n",transaction_split(&transaction_map1.1).unwrap());      
        let _=&maked_transaction.push(transaction_split(&transaction_map1.1).unwrap());            
    } 
    Ok(maked_transaction)
}

