    
//     // #[derive(Debug,Clone,Deserialize,Serialize)]
//     // pub enum Transactions {
//     //      puts
//     // }
//     // #[derive(Debug,Clone,Deserialize,Serialize)]
//     // pub enum Transaction {
//     //     to_addr,
//     //     value
//     // }

// #[cfg(test)]
// mod tests {
//     use std::{fs::{self, File}, io::BufReader};
//     use library_utils::slicer;
//     use serde_json::{json, Value,Result};
//     use serde::*;
//     use super::*;
//     use library_blockchain::{*, transaction::OptionTransaction};    
//     pub (crate) use library_blockchain::transaction::{Value as ModelValue};
 

//     #[derive(Debug,Deserialize,PartialEq)]
//     pub struct ModelTransaction {
//     inputs:Vec<ModelValue>,
//     outputs:Vec<ModelValue>        
//     }
//     #[derive(Debug,Deserialize,PartialEq)]
//     pub struct ModelBlock {
//     transactions:Vec<ModelTransaction>    
//     }
//     #[derive(Debug,Deserialize,PartialEq)]
//     pub struct Model {
//     blocks:Vec<ModelBlock>    
//     }
//     #[test]
//     fn untyped_example() -> Result<()> {
//             // Some JSON input data as a &str. Maybe this comes from the user.
//             let data=r#"
//             "transactions":[{
//                 "transaction1":[
//                     {
//                         "inputs":[{
//                             "to_addr": "Alice",
//                             "value": "47"                            
//                         },{
//                             "to_addr": "Bob",
//                             "value": "3"                            
//                         }],    
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "46"                            
//                         },{
//                             "to_addr": "Bob",
//                             "value": "1"                            
//                         }]    
//                     }
//                 ],              
//                 "transaction2":[
//                     {
//                        "inputs":[{                                                               
//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "0"
//                         }]
//                     }
//                 ]             
//             }]"#;
        
//             let m: ModelBlock = serde_json::from_str(data)?;
              
//             //println!("Model: {}", m);
//             //println!("Model: {} {}", m.inputs[0].to_addr,m.inputs[0].value);

//             // let modelvalue= ModelValue{            
//             //     to_addr: "Alice".to_string(),
//             //     value: "Ab".to_string()     
//             // };
//             // let mut vec_modelvalue=vec![];
//             // vec_modelvalue.push(modelvalue);
//             // let model= Model{            
//             //      inputs:  vec_modelvalue
//             // }; 
            
//             // assert_eq!(m,model);
            
//             Ok(())
//     }
     
// #[test]
// fn sample_trx_json_default()-> Result<()> 
// {    
//     let js=r#"
//     "transactions":[{
//         "transaction1":[
//             {
//                 "inputs":[{
//                     "to_addr": "Alice",
//                     "value": "47"                            
//                 },{
//                     "to_addr": "Bob",
//                     "value": "3"                            
//                 }],    
//                 "outputs":[{
//                     "to_addr": "Alice",
//                     "value": "46"                            
//                 },{
//                     "to_addr": "Bob",
//                     "value": "1"                            
//                 }]    
//             }
//         ],              
//         "transaction2":[
//             {
//                "inputs":[{                                                               
//                 }],
//                 "outputs":[{
//                     "to_addr": "Alice",
//                     "value": "0"
//                 }]
//             }
//         ]             
//     }]"#;
    
//         // let dt=serde_json::to_string(js).unwrap();
//     // let data:Value=serde_json::from_str(&dt).unwrap();    
//     // let trx_name_from_file="transaction1,transaction2".to_string();
//     // let serde_values_transactions:serde_json::Value= serde_json::from_reader(&data).unwrap();
//     let file = File::open("sample.json").unwrap();
//     let reader = BufReader::new(file);
//     // Read the JSON contents of the file as an
//     let serde_values_transactions:serde_json::Value= serde_json::from_reader(reader).unwrap();  
//     println!("{:?}",serde_values_transactions);
    
//     let trx_name_from_file="transaction1,transaction2".to_string();

//     println!("{}",&serde_values_transactions);

//     let mut transactions:Vec<OptionTransaction> = vec![];     
//     let list=slicer::split_comma_wordlist(&trx_name_from_file);
    
//     let values_transactions=serde_values_transactions["transactions"].clone(); 

//     println!("{:?}",&list);
//     if ! &values_transactions[0].is_null(){

//     for item in list {        

//         let mut trx_inputs_model_vec :Vec<ModelValue> = vec![];  
//         let trx=(values_transactions[0].as_object().unwrap()).get(item).unwrap();   
        
//         if !trx.is_null(){                
//             let trx_inputs=(trx[0].as_object().unwrap()).get("inputs").unwrap();                
                            
//             if !(trx_inputs.is_null()) && !(trx_inputs.as_array().is_none()){                    
//                 let trx_inputs_vec=trx_inputs.as_array().unwrap();                    
                

//                 for item_internal_inputs in trx_inputs_vec {                                                              
//                     let mut trx_inputs_model:ModelValue=ModelValue{
//                         to_addr: String::from(""),
//                         value: 0,
//                     };
                    
//                     if !item_internal_inputs.is_null(){
                    
//                         if !(item_internal_inputs["value"].is_null() && item_internal_inputs["to_addr"].is_null()){                      
//                                 trx_inputs_model= ModelValue{
//                                     to_addr:item_internal_inputs["to_addr"].as_str().unwrap().to_owned(),
//                                     value:item_internal_inputs["value"].as_str().unwrap().parse::<u64>().unwrap()
//                                 };
//                                 trx_inputs_model_vec.push(trx_inputs_model);                                       
//                         }
                        
//                 }
//                 }                    
//             }
                        
//                 let mut trx_outputs_model_vec :Vec<ModelValue> = vec![];  
//                 let trx_outputs=(trx[0].as_object().unwrap()).get("outputs").unwrap();                
                                
//                 if !(trx_outputs.is_null()) && !(trx_outputs.as_array().is_none()){                    
//                     let trx_outputs_vec=trx_outputs.as_array().unwrap();                    
                    

//                     for item_internal_outputs in trx_outputs_vec {                                                              
//                         let mut trx_outputs_model:ModelValue=ModelValue{
//                             to_addr: String::from(""),
//                             value: 0,
//                         };
                        
//                         if !item_internal_outputs.is_null(){
                        
//                             if !(item_internal_outputs["value"].is_null() && item_internal_outputs["to_addr"].is_null()){                      
//                                     trx_outputs_model= ModelValue{
//                                         to_addr:item_internal_outputs["to_addr"].as_str().unwrap().to_owned(),
//                                         value:item_internal_outputs["value"].as_str().unwrap().parse::<u64>().unwrap()
//                                     };
//                                     trx_outputs_model_vec.push(trx_outputs_model);                                        
//                             }
                            
//                     }
//                 }
                                                    
//                 let new_transaction= Transaction::new(trx_inputs_model_vec,trx_outputs_model_vec);
//                 transactions.push(new_transaction);                   
//         }         

//         }
//       }     
     
//     }       
//     Ok(())
// }

// }