
use std::{fmt, error::Error};

use library_blockchain::{*};
use serde_json::{json, Value};
use serde::{ser::{Serialize, SerializeStruct}, Deserializer};

/// 1. produce block, without minning and transactions
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
/// 
/// 2. block minning without transactions
///```no_run
// fn main () {    
//     let transaction:Transaction= Transaction {
//         inputs: vec![ ],
//         outputs: vec![]
//     };

//     let difficulty= 0x0fffffffffffffffffffffffffffffff;
//     let  mut block = Block::new(0,now(), vec![0; 32],vec![transaction],difficulty);//,"Genesis Block".to_owned()
        
//     //println!("{:?}",&block);
//     block.mine();
//     println!("Mined genesis block :{:?}",&block);    

//     let mut last_hash=block.hash.clone();

//     let mut blockchain= Blockchain::default();
//     blockchain.blocks.push(block);

//     for i in 1..=10 { 
//         let transaction:Transaction= Transaction {
//             inputs: vec![ ],
//             outputs: vec![]
//         };
//         let  mut block = Block::new(i,now(), last_hash,vec![transaction],difficulty);//,"Genesis Block".to_owned()        
//         //println!("{:?}",&block);
//         block.mine();
//         println!("Mined  block :{:?}",&block);  
//         last_hash= block.hash.clone();
//         blockchain.blocks.push(block);
//     }
// }
///```

// 3.All with transactions
#[allow(dead_code)]
#[allow(unused_mut)]
fn main () {
        
    let difficulty = 0x000fffffffffffffffffffffffffffff;    
//    let trx_serialized=
sample_trx_json();
    //dbg!(sample_trx_json().unwrap().iter().collect::<Vec<_>>());    

    
    //let v2= serde_json::from_str(trx_output_data2);
    //let v1 = trx_output_data1;
    //let v2= trx_output_data2;
    //let serialized = serde_json::to_string(&point).unwrap();


    let genesis_trx= Transaction::default();    
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![genesis_trx], difficulty);
    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("\n\nFailed to add genesis block");

    let sample_trx1= Transaction::new( vec![],vec![
            transaction::Value {
                to_addr: "Chris".to_owned(),
                value: 0,
            },
        ]);
    
    let sample_trx2= Transaction::new( vec![
        blockchain.blocks[0].option_transactions[0].puts.as_ref().unwrap().outputs[0].clone(),//if it to be 50 then we have to have sum(outputs)=50
    ],
    vec![
        transaction::Value {
            to_addr: "Removeable output.it is Name-for rise error InsufficientInputValue".to_owned(),
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
    let mut block = Block::new(1, now(), last_hash, vec![sample_trx1,sample_trx2], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    //last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("\n\nFailed to add block");
}



fn sample_trx_json() ->  Result<Vec<Transaction>,std::io::Error>{

    let mut transactions:Vec<Transaction> = vec![];             

    let json_transaction = json!({
    "transactions":[{
            "transaction1":[
                {
                    "inputs":[{          
                        "to_addr": "I",
                        "value": "0",   
                    }],
                    "outputs":[{
                        "to_addr": "Chris",
                        "value": "0",
                    }]
                }
            ],
            "transaction2":[
                {
                    "inputs":[{          
                        "to_addr": "II",
                        "value": "0",           
                    },{
                        "to_addr": "III",
                        "value": "47",    
                    }],
                    "outputs":[{
                                               
                    }]
                }
            ],
            "transaction3":[
                {
                    "inputs":[{                  
                    }],
                    "outputs":[{
                        "to_addr": "Alice",
                        "value": "47",                            
                    },{
                        "to_addr": "Bob",
                        "value": "3",                            
                    }]    
                }
            ]
        }]
    });
    

    let serde_values_transactions:serde_json::Value= serde_json::from_value(json_transaction).unwrap();
    let values_transactions=serde_values_transactions["transactions"].clone(); 
    //println!("{:#?}",values_transactions); 
    let list=["transaction1","transaction2","transaction3"];
    

    if(!values_transactions[0].is_null()){
        for item in list {        


            let mut trx_inputs_model_vec :Vec<ModelValue> = vec![];  
            let trx=(values_transactions[0].as_object().unwrap()).get(item).unwrap();   
            
            if(!trx.is_null()){                
                let trx_inputs=(trx[0].as_object().unwrap()).get("inputs").unwrap();                
                                
                if(!(trx_inputs.is_null()) && !(trx_inputs.as_array().is_none())){                    
                    let trx_inputs_vec=trx_inputs.as_array().unwrap();                    
                    

                    for item_internal_inputs in trx_inputs_vec {                                                              
                        let mut trx_inputs_model:ModelValue=ModelValue{
                            to_addr: String::from(""),
                            value: 0,
                        };
                        
                        if(!item_internal_inputs.is_null()){
                        
                            if (!(item_internal_inputs["value"].is_null() && item_internal_inputs["to_addr"].is_null())){                      
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
                                    
                    if(!(trx_outputs.is_null()) && !(trx_outputs.as_array().is_none())){                    
                        let trx_outputs_vec=trx_outputs.as_array().unwrap();                    
                        
    
                        for item_internal_outputs in trx_outputs_vec {                                                              
                            let mut trx_outputs_model:ModelValue=ModelValue{
                                to_addr: String::from(""),
                                value: 0,
                            };
                            
                            if(!item_internal_outputs.is_null()){
                            
                                if (!(item_internal_outputs["value"].is_null() && item_internal_outputs["to_addr"].is_null())){                      
                                        trx_outputs_model= ModelValue{
                                            to_addr:item_internal_outputs["to_addr"].as_str().unwrap().to_owned(),
                                            value:item_internal_outputs["value"].as_str().unwrap().parse::<u64>().unwrap()
                                        };
                                        trx_outputs_model_vec.push(trx_outputs_model);                                        
                                }
                                
                        }
                    }
                    
                  
                    
                    // let new_transaction= Transaction::new( vec![
                    //     transaction::Value {
                    //         to_addr: trx_inputs_model.to_addr,
                    //         value: trx_inputs_model.value,
                    //     },
                    // ],vec![
                    //     transaction::Value {
                    //         to_addr: trx_outputs_model.to_addr,
                    //         value: trx_outputs_model.value,
                    //     },
                    // ]);
                    // transactions.push(new_transaction.puts.unwrap());                   
              }         
              
              println!("Printed:{:?}",trx_inputs_model_vec);
              println!("-----------------");
              println!("Printed:{:?}",trx_outputs_model_vec);
              
            }
        }     
    }       
        Ok(transactions)
}


#[derive(serde::Deserialize,serde::Serialize,Debug,Clone)]
 struct ModelValue {
     to_addr: String,
     value: u64     
}

#[derive(Debug)] // Allow the use of "{:?}" format specifier
enum CustomError {
    //o(IoError),
    //Utf8(Utf8Error),
    StringParse(std::string::ParseError),
    SerdeJson(serde_json::Error),
    Other,
}

impl CustomError{
  
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

// Allow this type to be treated like an error
impl std::error::Error for CustomError{
    fn description(&self) -> &str {
        match *self {
            CustomError::StringParse(ref cause) => cause.description(),
            CustomError::SerdeJson(ref cause) => cause.description(),
            CustomError::Other => "Unknown error!",
        }
    }
// Use an Option<&Error>. This is the return type of Error.cause().
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




//# ```compile_fail  /// ```should_panic    /// ```edition2018  /// ```ignore