
use super::*;
use library_blockchain::transaction::{Value as ModelValue, OptionTransaction};
use library_blockchain::{*};

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn blockchain_factory<F>(difficulty:u128, f : F) -> Result<Blockchain,CustomError>
                      where   
                      F: FnOnce()->  Result<serde_json::Value,CustomError>     
{    
    let serde_values_transactions:serde_json::Value= serde_json::from_value(f().unwrap()).unwrap();
    let mut blockchain = Blockchain::new();
    let mined_block_vec=sample_trx_json_default_fetch(difficulty,serde_values_transactions).unwrap();
    if mined_block_vec.len()<1{
        return Err(CustomError::BlockchainFactory)
    }
    for block in mined_block_vec{
        blockchain.update_with_block(block).expect("\n\nFailed to add block");
    }
    Ok(blockchain)
}
fn sample_trx_json_default_fetch(difficulty:u128,serde_values_transactions:serde_json::Value) -> Result<Vec<Block>,CustomError>
{       
    let blocks_val:serde_json::Value=serde_values_transactions["blocks"].clone();   

    let len_blocks_map=blocks_val[0].as_object().unwrap();       
    let values_blocks_found_len=len_blocks_map.len();   
    if blocks_val.to_string().find("block")==None || values_blocks_found_len< 1usize{
        return Err(CustomError::BlockchainFactory)
    }
    //let values_blocks_found_len=blocks_val.to_string().find("block").unwrap();
    let mut transactions:Vec<OptionTransaction> = vec![];  
    let mut values_transactions_found:Vec<String>=vec![];
    let mut blocks:Vec<Block>=vec![];
    
    println!("**Len block:**\n{:?}\n",values_blocks_found_len);

    for i in 0..values_blocks_found_len{
        println!("**i:**{:?}\n",&i);
        let iblock:String=concat_string!("block",String::from((&i+1).to_string()).as_str());
        let blocks_map=blocks_val[0].as_object().unwrap();   
        let block_str=concat_string!("block",(&i+1).to_string());
        let block_map=blocks_map.get_key_value(&block_str);
         
    
        for block in block_map{
            let mut maked_transaction:Vec<OptionTransaction> = vec![];  
            println!("**block:**\n{:?}\n",&block);
            let y=block.1;
            let transactions_map:Vec<serde_json::Value>=block.1.as_array().unwrap().to_vec();    
            let temp_map_for_getting_len=&transactions_map[0]["transactions"].clone();
            let values_transactions_found_len=(temp_map_for_getting_len[0].as_object().unwrap()).len();                      
            
            let maked_transaction:Vec<OptionTransaction>= fetch_raw_block_transactions(transactions_map,values_transactions_found_len).unwrap();
            let new_block:Block=making_blocks(i as u32,maked_transaction,difficulty).unwrap();
            println!("**new_block:**\n{:?}\n\n",&new_block);
            let _= &blocks.push(new_block);
        }
        //println!("***block_map:***\n{:?}\n\n",block_map);          
        //let values_transactions_found_len=transactions_val.to_string().find("transaction").unwrap();
        //println!("values_transactions_found_len:{:?}",values_transactions_found_len);        
    }   
    Ok(blocks) 
}          
 
fn making_blocks(index:u32,maked_trx_vec:Vec<OptionTransaction>, difficulty:u128) -> Result<Block,CustomError>{
    
    if maked_trx_vec.len()==0 {
        return Err(CustomError::BlockchainFactory)
    }

    let mut maked_block = Block::new(index, now(), vec![0; 32], maked_trx_vec, difficulty);
    maked_block.mine();    

    Ok(maked_block)
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

