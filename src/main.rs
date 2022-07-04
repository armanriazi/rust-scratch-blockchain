#![deny(rust_2018_idioms)]
#![warn(rust_2018_idioms)]

use env_logger::{Builder, Target};

use crate::factory::blockchain_factory;
use library_blockchain::*;
mod  library_utils;
use std::env;
use std::env::set_var;
use std::fs::File;
use std::io::BufReader;
use log::{log_enabled, info, Level};

pub mod factory;
pub mod sample;


//#[deny(elided_lifetimes_in_paths)]
#[allow(dead_code)]
#[allow(unused_mut)]
#[macro_use]
//extern crate library_utils;
extern crate log;
#[macro_use(concat_string)]
extern crate concat_string;

/// DIFFICULTY=0x000fffffffffffffffffffffffffffff cargo run
/// RUST_LOG=INFO DIFFICULTY=0x00000fffffffffffffffffffffffffff time cargo run file sample-bolocks.json 
fn main() -> Result<(), CustomError> {

  
    init_env_logger(true);

    info!("Starting Up...");


    let difficulty_str = var_ret_difficulty("0x00ffffffffffffffffffffffffffffff");
    if difficulty_str.is_empty() {
        set_var("DIFFICULTY", "0x00ffffffffffffffffffffffffffffff");
    }

    let difficulty = string_to_u128(&difficulty_str);
    let mut args: Vec<String> = env::args().collect();
    let mut mode = String::default();
    let mut file_name = String::default();
    let blockchain = Blockchain::new();

    if (&args).len() <= 1 {
        println!("** Please select a runner mode\n Help(file path transaction_list, or macrojson transaction_list)\n Default is cargo run macrojson **\n");
        args.push("macrojson".to_owned());
    } else {
        mode = (&args[1]).trim().to_lowercase();
    }
    if &mode == "file" {
        file_name = (&args[2]).trim().to_lowercase();
        let file = File::open(&file_name).unwrap();
        println!("**************************************************************");
        blockchain_factory(blockchain, difficulty, || {
            sample_trx_json_data_block_from_file(&file)
        })?;
                
    } else if &mode == "macrojson" {
        println!("Selected mode is macrojson\n");
        blockchain_factory(blockchain, difficulty, || {
            sample::sample_trx_json_data_from_module()
        })?;
        
    } else if &mode == "stringjson" {
        println!("Selected mode is stringjson\n");
        blockchain_factory(blockchain, difficulty, || {
            sample::sample_trx_json_data_from_string()
        })?;
        
    } else {
        println!("The mode is not selected! Default is macrojson\n");
        blockchain_factory(blockchain, difficulty, || {
            sample::sample_trx_json_data_from_module()
        })?;
        
    }
    Ok(())
}

fn init_env_logger(is_enable:bool) {
        
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    if is_enable{
       builder.init();
    }
        
    if log_enabled!(Level::Info) {            
        info!("------------Welcome to env_logger------------");
    }
    else  {
        println!("----------env_logger have not been activated----------");
    }
}

fn sample_trx_json_data_block_from_file(file: &File) -> Result<serde_json::Value, CustomError> {
    println!("Selected mode is file!");

    let reader = BufReader::new(file);
    let serde_values_transactions = serde_json::from_reader(reader)?;
    

    return Ok(serde_values_transactions);
}

fn var_ret_difficulty(difficulty_arg: &str) -> String {
    match env::var("DIFFICULTY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("\n🦀{e}! We used default difficulty🦀\n");
            env::var("DIFFICULTY").unwrap_or(difficulty_arg.to_owned())
        }
    }
}
