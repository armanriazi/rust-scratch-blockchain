use std::fs::File;
use std::io::BufReader;
use std::{env};
use library_blockchain::{*};
use library_utils::{stringtou128::string_to_u128};
use std::env::{set_var};
use crate::factory::blockchain_factory;
pub  mod factory;
pub  mod sample;

#[allow(dead_code)]
#[allow(unused_mut)]
#[macro_use(concat_string)]
extern crate concat_string;

fn main() -> Result<(), CustomError> {
    let  difficulty_str=var_ret_difficulty("0x00ffffffffffffffffffffffffffffff"); 
    if difficulty_str.is_empty(){
        set_var("DIFFICULTY", "0x00ffffffffffffffffffffffffffffff");
    }

    let difficulty=string_to_u128(&difficulty_str);
    let mut args: Vec<String> = env::args().collect();
    let mut mode=String::default();
    let mut file_name=String::default();
    

    if  (&args).len()<=1 {
        println!("** Please select a runner mode\n Help(file path transaction_list, or module transaction_list)\n Default is cargo run module **\n");        
        args.push("module".to_owned());
    }
    else {
        mode=(&args[1]).trim().to_lowercase();        
    }
    if  &mode =="file" {          
            file_name=(&args[2]).trim().to_lowercase();                    
            let file = File::open(&file_name).unwrap();
            println!("**************************************************************");            
            blockchain_factory(difficulty,|| sample_trx_json_data_block_from_file(&file)).unwrap(); 
            //println!("\nBlockchain:\n{:?}", &blockchain);   
        }        
    else if &mode=="module" {           
            println!("Selected mode is module\n");   
            blockchain_factory(difficulty,|| sample::sample_trx_json_data_from_module()).unwrap();              
        }
    else{         
            println!("The mode is not selected! Default is module\n");   
            blockchain_factory(difficulty,|| sample::sample_trx_json_data_from_module()).unwrap();
    }
    //println!("**Maked Blockchain:**\n{:?}\n",&blockchain.blocks);
    Ok(())
}


fn sample_trx_json_data_block_from_file(file:&File) -> Result<serde_json::Value, CustomError>{

    println!("Selected mode is file!");    

    let reader = BufReader::new(file);  
    let serde_values_transactions=serde_json::from_reader(reader).unwrap();            
    
    return Ok(serde_values_transactions)
}


fn var_ret_difficulty(difficulty_arg:&str)-> String{
    match env::var("DIFFICULTY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("\n🦀{e}! We used default difficulty🦀\n");
            env::var("DIFFICULTY").unwrap_or(difficulty_arg.to_owned())
      }
    }  
}
