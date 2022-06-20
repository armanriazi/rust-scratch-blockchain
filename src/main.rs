
use std::io::{BufRead, BufReader, Write};
use std::net::ToSocketAddrs;
use std::str::from_utf8;
use std::time::Duration;
use std::{fmt, env, fs};
use library_blockchain::{*};
use library_blockchain::transaction::{Value as ModelValue, OptionTransaction};
use serde::__private::de::IdentifierDeserializer;
use std::env::{var,set_var};
use serde_json::{json};

pub  mod sample;
/// 1. Produce block, without minning and transactions
///```no_run
/// fn main () {
///     //index: u32, timestamp: u128, prev_block_hash: Hash, transactions: Vec<Transaction>, difficulty: u128
///     let transaction:Transaction= Transaction {
///         inputs: vec![ ],
///         outputs: vec![]
///     };
///     let  mut block = Block::new(0,now(), vec![0; 32],vec![transaction],0);//,"Genesis Block".to_owned()
///     println!("{:?}",&block);
///     let h=block.hash();
///     println!("Printed:{:?}",&h);
///     block.hash=h;
///     println!("Printed:{:?}",&block);
/// }

/// 2. Block minning without transactions
///```no_run
/// fn main () {    
///    let transaction:Transaction= Transaction {
///        inputs: vec![ ],
///        outputs: vec![]
///    };/
///    let difficulty= 0x0fffffffffffffffffffffffffffffff;
///    let  mut block = Block::new(0,now(), vec![0; 32],vec![transaction],difficulty);//,"Genesis Block".to_owned()     
///    //println!("{:?}",&block);
///    block.mine();
///    println!("Mined genesis block :{:?}",&block);    /
///    let mut last_hash=block.hash.clone();/
///    let mut blockchain= Blockchain::default();
///    blockchain.blocks.push(block);/
///    for i in 1..=10 { 
///        let transaction:Transaction= Transaction {
///            inputs: vec![ ],
///            outputs: vec![]
///        };
///        let  mut block = Block::new(i,now(), last_hash,vec![transaction],difficulty);//,"Genesis Block".to_owned()        
///        //println!("{:?}",&block);
///        block.mine();
///        println!("Mined  block :{:?}",&block);  
///        last_hash= block.hash.clone();
///        blockchain.blocks.push(block);
///    }
///}
///```

/// 3. Almost of features supported


#[allow(dead_code)]
#[allow(unused_mut)]
fn main () {

    set_var("DIFFICULTY", "0x000fffffffffffffffffffffffffffff");
    let  difficulty_str=var_ret_difficulty("0x000fffffffffffffffffffffffffffff"); 
    
    let diff_str = difficulty_str.trim().to_lowercase().to_string();
    let diff_digits = diff_str.strip_prefix("0x").unwrap();
    let difficulty = u128::from_str_radix(diff_digits, 16).unwrap();
    let diff_bytes = difficulty.to_le_bytes();
    
    let de_diff_bytes = diff_bytes;
    let de_diff = u128::from_le_bytes(de_diff_bytes);
    let de_diff_str = format!("0x{de_diff:032x}");

    assert_eq!(diff_str, de_diff_str);

    let mut args: Vec<String> = env::args().collect();
    let mut transactions_block2:Vec<OptionTransaction>=vec![];

    // println!("Seleceted Mode On: {}", &args[1]);
    // println!("Transaction Names List are: {}", &args[2]);    
    // println!("By File Name: {}", &args[3]);

    if  args.len()<=1 {
        println!("Please select a runner mode\n Help(file path transaction_list, object transaction_list, or module transaction_list)\n Default is cargo run object transation1,transaction2");        
        args.push("object".to_owned());
        args.push("transation1,transaction2".to_owned());        
        transactions_block2= sample_trx_object_default().unwrap();  
    }
    if  args[1].as_str()=="file" {        
            let file_contents = fs::read_to_string(&args[3])
            .expect("Something went wrong reading the file");
            println!("******************************\n");
            println!("With text:\n{}", file_contents);    
            println!("******************************\n");
            transactions_block2=sample_trx_json_default(&args[3],|| sample_trx_json_data_block2_from_file(&file_contents)).unwrap();       
        }
        else if args[1].as_str()=="object" {                  
             transactions_block2= sample_trx_object_default().unwrap();            
        }
        else if args[1].as_str()=="module" {                    
             transactions_block2=sample_trx_json_default(&args[2],|| sample::sample_trx_json_data_block2()).unwrap();   
        }
        else{
         println!("something else!");
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
}

fn read_le_u128(input: &mut &[u8]) -> u128 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u128>());
    *input = rest;
    u128::from_le_bytes(int_bytes.try_into().unwrap())
}


fn sample_trx_json_default<F>(trx_name_from_file:&String, f : F) -> Result<Vec<OptionTransaction>,std::io::Error>
where
        F: FnOnce()->  Result<serde_json::Value,std::io::Error>     
    {    
    let serde_values_transactions:serde_json::Value= serde_json::from_value(f().unwrap()).unwrap();
    let values_transactions=serde_values_transactions["transactions"].clone(); 

    let mut transactions:Vec<OptionTransaction> = vec![];     
    let list=library_utils::ustringslicer::split_comma_wordlist(trx_name_from_file);
    

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
//   let err = std::fmt::Error::Err("NaN".parse::<u32>());
//   println!("Printed:{:?}",err);
 
    Ok(transactions)
}

fn sample_trx_object_default() ->  Result<Vec<OptionTransaction>,std::io::Error>{

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


pub fn sample_trx_json_data_block2_from_file(file:&str) -> Result<serde_json::Value,std::io::Error>{
    return Ok(json!(&file))
}


pub fn var_ret_difficulty(difficulty_arg:&str)-> String{
    var("DIFFICULTY").unwrap_or(difficulty_arg.to_owned())    
}

// fn sample_trx_json_data_genesis_block() -> Result<serde_json::Value,std::io::Error>{
//     return Ok(json!({
//         "transactions":[{
//                 "transaction0":[
//                     {
//                         "inputs":[{                                     
//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "50",
//                         },
//                         {
//                             "to_addr": "Bob",
//                             "value": "10",
//                         }]
//                     }
//                 ],
//             }]
//         }))
// }


#[derive(Debug)] // Allow the use of "{:?}" format specifier
enum CustomError {
    //o(IoError),
    //Utf8(Utf8Error),
    StringParse(std::string::ParseError),
    SerdeJson(serde_json::Error),
    Other,
}


// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::StringParse(ref cause) => write!(f, "StringParse Error: {}", cause),
            CustomError::SerdeJson(ref cause) => write!(f, "SerdeJson Error: {}", cause),
            CustomError::Other => write!(f, "Unknown error!"),
        }
    }
}
impl std::error::Error for CustomError{
    // fn source(&self) -> Option<&(dyn std::error::Error)> {
    //     match *self {
    //         CustomError::StringParse(ref cause) => Some(cause),
    //         CustomError::SerdeJson(ref cause) => Some(cause),
    //         CustomError::Other => None,
    //     }
    // }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CustomError::StringParse(ref cause) => Some(cause),
            CustomError::SerdeJson(ref cause) => Some(cause),
            CustomError::Other => None,
        }
    }


}
impl From<std::string::ParseError> for CustomError {
    fn from(cause: std::string::ParseError) -> CustomError {
        CustomError::StringParse(cause)
    }
}
impl From<serde_json::Error> for CustomError {
    fn from(cause: serde_json::Error) -> CustomError {
        CustomError::SerdeJson(cause)
    }
}



//-----------------------Commnents---------------------//
// #[derive(serde::Deserialize,serde::Serialize,Debug,Clone)]
//  struct ModelValue {
//      to_addr: String,
//      value: u64     
// }

    // let trx1_output = "{
    //     \"to_addr\": \"Chris\",
    //     \"value\": \"0\"        
    // }";
// let mut output_values:Vec<ModelValue> = vec![];
//     let trx1_output_value:ModelValue=serde_json::from_str(&trx1_output).unwrap();
//     output_values.push(trx1_output_value);    


// impl Serialize for ModelValue {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_struct("ModelValue", 1)?;
//         s.serialize_field("to_addr", &self.to_addr)?;
//         s.serialize_field("value", &self.value)?;        
//         s.end()
//     }
// }





    // let trx2_output = r#"
    // {
    //     "to_addr": "Removeable output.it is Name-for rise error InsufficientInputValue",
    //     "value": 0
    // }"#;

    // let trx2_output = r#"
    // {
    //     "to_addr": "Alice",
    //     "value": 47
    // }"#;

    // let trx2_output = r#"
    // {
    //     "to_addr": "Bob",
    //     "value": 3
    // }"#;




    // #[derive(PartialEq, Debug)]
// struct Difficulty(u128);

// impl fmt::UpperHex for Difficulty {
//     fn fmt(mut self: &Difficulty, f: &mut fmt::Formatter) -> fmt::Result {
//         //let bytes = self.0.as_bytes().to_vec();    
//         //let  difficulty=difficulty_bytes_as_u128(&bytes);           
//     //  unsafe {
//     //     //std::mem::transmute(NumUu as u128);
//     //     NumUu+=difficulty;
//     //    // NumUu=*(difficulty as *const u128)
//     // };

//     //let val = self.0;
//     //fmt::UpperHex::fmt(&val, f)

//         let hexa = u128::from(self.0);
//         write!(f, "#{:X}", hexa)
//     }
// }

// impl fmt::Display for Difficulty {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {            
//             Difficulty::Num(num) => write!(f,"{:X}",num), // <4>
//         }
//         }
// }
//# ```compile_fail  /// ```should_panic    /// ```edition2018  /// ```ignore