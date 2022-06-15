

use library_blockchain::{*, transaction::OptionTransaction};
use serde_json::{Result, Value};

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
    let trx1_outputs=sample_trx_json();
    println!("{:#?}", trx1_outputs.unwrap());
    //let v2= serde_json::from_str(trx_output_data2);
    //let v1 = trx_output_data1;
    //let v2= trx_output_data2;
    //let serialized = serde_json::to_string(&point).unwrap();


    let genesis_trx= Transaction::default();    
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![genesis_trx], difficulty);
    genesis_block.mine();

    //println!("Mined genesis block {:?}", &genesis_block);

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


fn sample_trx_json() -> Result<Vec<ModelValue>> {
    let trx1_output = "{
        \"to_addr\": \"Chris\",
        \"value\": \"0\"
    }";

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

    let mut output_values:Vec<ModelValue> = vec![];
    let trx1_output_value:ModelValue=serde_json::from_str(trx1_output).unwrap();
    output_values.push(trx1_output_value);    

    Ok(output_values)

}


#[derive(serde::Deserialize, Debug)]
pub struct ModelValue {
    pub to_addr: String,
    pub value: String,
}
//# ```compile_fail  /// ```should_panic    /// ```edition2018  /// ```ignore