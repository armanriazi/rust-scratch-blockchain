#![deny(rust_2018_idioms)]
#![warn(rust_2021_idioms)]

use env_logger::{Builder, Target};
use proc_macro;
use crate::factory::blockchain_factory;
use library_blockchain::*;
use library_utils::stringtou128::string_to_u128;
use std::env;
use std::env::set_var;
use std::fs::File;
use std::io::BufReader;
use log::{debug, error, log_enabled, info, Level,trace};
//use concat_string;
pub mod factory;
pub mod sample;


#[deny(elided_lifetimes_in_paths)]
#[allow(dead_code)]
#[allow(unused_mut)]
#[macro_use]
extern crate log;
#[macro_use(concat_string)]
extern crate concat_string;

fn main() -> Result<(), CustomError> {

    //env_logger::init();
    init_env_logger(true);

    info!("starting up");


    let difficulty_str = var_ret_difficulty("0x00ffffffffffffffffffffffffffffff");
    if difficulty_str.is_empty() {
        set_var("DIFFICULTY", "0x00ffffffffffffffffffffffffffffff");
    }

    let difficulty = string_to_u128(&difficulty_str);
    let mut args: Vec<String> = env::args().collect();
    let mut mode = String::default();
    let mut file_name = String::default();
    let mut blockchain = Blockchain::new();

    if (&args).len() <= 1 {
        println!("** Please select a runner mode\n Help(file path transaction_list, or module transaction_list)\n Default is cargo run module **\n");
        args.push("module".to_owned());
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
        //.unwrap();
        //println!("\nBlockchain:\n{:?}", &blockchain);
    } else if &mode == "macrojson" {
        println!("Selected mode is macrojson\n");
        blockchain_factory(blockchain, difficulty, || {
            sample::sample_trx_json_data_from_module()
        })?;
        //.unwrap();
    } else if &mode == "stringjson" {
        println!("Selected mode is stringjson\n");
        blockchain_factory(blockchain, difficulty, || {
            sample::sample_trx_json_data_from_string()
        })?;
        //.unwrap();
    } else {
        println!("The mode is not selected! Default is macrojson\n");
        blockchain_factory(blockchain, difficulty, || {
            sample::sample_trx_json_data_from_module()
        })?;
        //.unwrap();
    }
    
    //println!("**Maked Blockchain:**\n{:?}\n",&blockchain.blocks);
    Ok(())
}

fn init_env_logger(is_enable:bool) {
        
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    if is_enable{
       &builder.init();
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
    //.unwrap();

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

// pub fn shave_the_yak(yak: &mut Yak) {
//     trace!(target = "yak_events", yak = as_serde!(yak); "Commencing yak shaving");

//     loop {
//         match find_a_razor() {
//             Ok(razor) => {
//                 info!(razor = razor; "Razor located");
//                 yak.shave(razor);
//                 break;
//             }
//             Err(err) => {
//                 warn!(err = as_error!(err); "Unable to locate a razor, retrying");
//             }
//         }
//     }
// }