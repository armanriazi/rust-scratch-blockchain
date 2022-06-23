#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, io::BufReader, collections::HashMap};
    use serde_json::{json, Value,Result};
    use serde::*;
    use super::*;

    #[derive(Debug,Deserialize,PartialEq)]
    pub struct ModelValue {        
    to_addr:String,
    value:String        
    }
    #[derive(Debug,Deserialize,PartialEq)]
    pub struct ModelTransaction {
    inputs:Vec<ModelValue>,
    outputs:Vec<ModelValue>        
    }
    #[derive(Debug,Deserialize,PartialEq)]
    pub struct ModelBlock {
    transactions:Vec<ModelTransaction>    
    }
    #[derive(Debug,Deserialize,PartialEq)]
    pub struct Model {
    blocks:Vec<ModelBlock>    
    }

#[test]
fn sample_trx_json_data_block2_from_file() -> Result<()> {
    
        println!("Selected mode is file!");    
        let file_name=("sample-blocks.json".to_string()).trim().to_lowercase();                    
        let file = File::open(&file_name).unwrap();
        let reader = BufReader::new(file);  
        let mut serde_values_transactions: HashMap<String,serde_json::Value>=serde_json::from_reader(reader).unwrap();      
        let x=&serde_values_transactions["blocks"];
        let y=&x[0]["block2"];
        let z=y.as_str().unwrap();
        let f=z.find("transaction1");
        dbg!("Printed {}",Some(f));
       // println!("Printed:{:?}", serde_values_transactions["blocks"]);

        //let nx=ary.into_iter().next().unwrap();
        //println!("Printed:{:?}", nx);



        //let trx_outputs:Value=(values_blocks[0].as_object().unwrap()).get("transactions").unwrap();
        // for block in values_blocks[0].as_array().into_iter().next(){
        //     println!("Printed:{:?}", &block.to_vec());
        // }
        
        //serde_json::from_value(value)
        //println!("Printed:{:?}",serde_values_transactions);
        
       // return Ok(serde_values_transactions)
    Ok(())
    }
}      

