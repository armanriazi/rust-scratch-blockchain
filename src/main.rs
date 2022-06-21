use std::{env, fs};
use library_blockchain::{*};
use library_blockchain::transaction::{Value as ModelValue, OptionTransaction};
use library_utils::{slicer, stringtou128::string_to_u128};
use std::env::{set_var};
use serde_json::{json};
pub  mod sample;

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() -> Result<(), CustomError> {
    let  difficulty_str=var_ret_difficulty("0x00ffffffffffffffffffffffffffffff"); 
    if difficulty_str.is_empty(){
        set_var("DIFFICULTY", "0x00ffffffffffffffffffffffffffffff");
    }

    let difficulty=string_to_u128(&difficulty_str);
    

    let mut args: Vec<String> = env::args().collect();
    let mut transactions_block2:Vec<OptionTransaction>=vec![];

    let mut mode=String::default();
    let mut trx_name=String::default();
    let mut file_name=String::default();

    if  (&args).len()<=1 {
        println!("Please select a runner mode\n Help(file path transaction_list, object transaction_list, or module transaction_list)\n Default is cargo run object transation1,transaction2");        
        args.push("object".to_owned());
        args.push("transation1,transaction2".to_owned());        
        transactions_block2= sample_trx_object_default()?;  
    }
    else {
        mode=(&args[1]).trim().to_lowercase();
        trx_name=(&args[2]).trim().to_lowercase();
        file_name=(&args[3]).trim().to_lowercase(); 
    }
    if  &mode =="file" {                
            let file_contents = fs::read_to_string(&file_name)
            .expect("Something went wrong reading the file");
            //println!("******************************\n");
            //println!("With text:\n{}", &file_contents);    
            println!("**************************************************************");
            transactions_block2=sample_trx_json_default(&trx_name,|| sample_trx_json_data_block2_from_file(&file_contents)).unwrap();       
        }
        else if &mode=="object" {                              
             transactions_block2= sample_trx_object_default()?;            
        }
        else if &mode=="module" {                    
             transactions_block2=sample_trx_json_default(&trx_name,|| sample::sample_trx_json_data_block2())?;   
        }
        else{
         println!("The mode is not selected!");
    }
    
    let transactions_genesis_block= vec![Transaction::default()];   
   //let transactions_genesis_block=sample_trx_json_default(transaction_name_list,|| sample_trx_json_data_genesis_block()).unwrap();    

    let mut genesis_block = Block::new(0, now(), vec![0; 32], transactions_genesis_block, difficulty);
    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("\n\nFailed to add genesis block");    
  
    let mut block = Block::new(1, now(), last_hash,transactions_block2, difficulty);
    block.mine();

    println!("Mined block {:?}", &block);

    //last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("\n\nFailed to add block");

    Ok(())
}


#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused)]
fn sample_trx_json_default<F>(trx_name_from_file:&String, f : F) -> Result<Vec<OptionTransaction>,CustomError>
        where
        F: FnOnce()->  Result<serde_json::Value,CustomError>     
    {    
    let serde_values_transactions:serde_json::Value= serde_json::from_value(f().unwrap()).unwrap();
   
    let values_transactions=serde_values_transactions["transactions"].clone(); 

    let mut transactions:Vec<OptionTransaction> = vec![];     
    let list=slicer::split_comma_wordlist(trx_name_from_file);
    
    println!("{:?}",&list);
    if ! &values_transactions[0].is_null(){

        for item in list {        

            let mut trx_inputs_model_vec :Vec<ModelValue> = vec![];  
            let trx=(values_transactions[0].as_object().unwrap()).get(item).unwrap();   
            
            if !trx.is_null(){                
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
                                                          
                    let new_transaction= Transaction::new(trx_inputs_model_vec,trx_outputs_model_vec);
                    transactions.push(new_transaction);                   
              }         

            }
        }     
    }       

 
    Ok(transactions)
}

fn sample_trx_object_default() ->  Result<Vec<OptionTransaction>,CustomError>{
    println!("Selected mode is object!");
    let mut transactions:Vec<OptionTransaction> = vec![];     
    
    let sample_trx2= Transaction::new( 
        vec![            
        ],vec![
            transaction::Value {
                to_addr: "Alex".to_owned(),
                value: 0,
            },
            transaction::Value {
                to_addr: "Alice".to_owned(),
                value: 47,
            },
            transaction::Value {
                to_addr: "Bob".to_owned(),
                value: 3
            },
        ]);    

    transactions.push(sample_trx2);

    Ok(transactions) 
}


pub fn sample_trx_json_data_block2_from_file(file_contents:&str) -> Result<serde_json::Value, CustomError>{
    println!("Selected mode is file!");
    return Ok(json!(&file_contents))
}


pub fn var_ret_difficulty(difficulty_arg:&str)-> String{
    match env::var("DIFFICULTY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("🦀{e}! We used default difficulty🦀");
            env::var("DIFFICULTY").unwrap_or(difficulty_arg.to_owned())
      }
    }  
}
