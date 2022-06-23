

#[allow(dead_code)]
#[allow(unused_mut)]
#[macro_use(concat_string)]
extern crate concat_string;
#[cfg(test)]
mod tests {
use super::*;
use std::fs::File;
use std::io::BufReader;
use library_blockchain::{*};
use library_blockchain::transaction::{Value as ModelValue, OptionTransaction};
use serde_json::json;

#[test]
fn sample_trx_json_default_fetch()-> (){    
    
let file = File::open("sample-blocks.json").unwrap();
let reader = BufReader::new(file);  
let serde_values_transactions:serde_json::Value=serde_json::from_reader(reader).unwrap();  

let blocks_val:serde_json::Value=serde_values_transactions["blocks"].clone();          
let values_blocks_found_len=blocks_val.to_string().find("block").unwrap();
let mut transactions:Vec<OptionTransaction> = vec![];  
let mut values_transactions_found:Vec<String>=vec![];

for i in 0..values_blocks_found_len{
    let iblock:String=concat_string!("block",String::from(i.to_string()).as_str());

        
    let transactions_val=blocks_val[i]["transactions"].clone();     
    println!("Printed:{:?}",&transactions_val);

    let values_transactions_found_len=transactions_val.to_string().find("transaction").unwrap();
    println!("Printed:{:?}",values_transactions_found_len);
    
    }     
}          
}



// for j in 0..values_transactions_found_len{
//     values_transactions_found=Vec::new();          
//     //let block_str=String::from(i.to_string()).as_str();
//     let concated_trx:String=concat_string!("transaction",String::from(j.to_string()).as_str());
//     values_transactions_found.push(concated_trx);
    
//     for item in values_transactions_found.into_iter().next() {        
    
//      if !transactions_val[0].is_null(){    
//         let mut trx_inputs_model_vec :Vec<ModelValue> = vec![];  
//         let trx=(transactions_val[0].as_object().unwrap()).get(item).unwrap();   
        
//         if !trx.is_null(){                
//             let trx_inputs=(trx[0].as_object().unwrap()).get("inputs").unwrap();                
                            
//             if !(trx_inputs.is_null()) && !(trx_inputs.as_array().is_none()){                    
//                 let trx_inputs_vec=trx_inputs.as_array().unwrap();                    
                
//                 for item_internal_inputs in trx_inputs_vec {                                                              
//                         let mut trx_inputs_model:ModelValue=ModelValue{
//                             to_addr: String::from(""),
//                             value: 0
//                     };
                    
//                     if !item_internal_inputs.is_null(){
                    
//                         if !(item_internal_inputs["value"].is_null() && item_internal_inputs["to_addr"].is_null()){                      
//                                 trx_inputs_model= ModelValue{
//                                     to_addr:item_internal_inputs["to_addr"].as_str().unwrap().to_owned(),
//                                     value:item_internal_inputs["value"].as_str().unwrap().parse::<u64>().unwrap()
//                                 };
//                                 trx_inputs_model_vec.push(trx_inputs_model);                                       
//                         }
                        
//                     }
//                 }                    
//             }
                        
//             let mut trx_outputs_model_vec :Vec<ModelValue> = vec![];  
//             let trx_outputs=(trx[0].as_object().unwrap()).get("outputs").unwrap();                
                            
//             if !(trx_outputs.is_null()) && !(trx_outputs.as_array().is_none()){                    
//                 let trx_outputs_vec=trx_outputs.as_array().unwrap();                    
                
//                 for item_internal_outputs in trx_outputs_vec {                                                              
//                     let mut trx_outputs_model:ModelValue=ModelValue{
//                         to_addr: String::from(""),
//                         value: 0,
//                     };
                    
//                     if !item_internal_outputs.is_null(){
                    
//                             if !(item_internal_outputs["value"].is_null() && item_internal_outputs["to_addr"].is_null()){                      
//                                     trx_outputs_model= ModelValue{
//                                         to_addr:item_internal_outputs["to_addr"].as_str().unwrap().to_owned(),
//                                         value:item_internal_outputs["value"].as_str().unwrap().parse::<u64>().unwrap()
//                                     };
//                                     trx_outputs_model_vec.push(trx_outputs_model);                                        
//                             }                                
//                         }
//                     }
//                 }                                   
//                 let new_transaction= Transaction::new(trx_inputs_model_vec,trx_outputs_model_vec);
//                 transactions.push(new_transaction);                   
//             }         
//         }
//     }
