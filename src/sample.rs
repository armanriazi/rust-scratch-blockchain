use serde_json::{json};

use crate::CustomError;

pub fn sample_trx_json_data_from_module() -> Result<serde_json::Value,CustomError>{
    
     Ok(json!(
        {
            "blocks":[{    
                "block1":[{
                    "transactions":[{
                        "transaction1":[
                            {
                                    "inputs":[{
                                                    
                                    }],    
                                    "outputs":[{
                                        "to_addr": "Alice",
                                        "value": "50"                           
                                    },{
                                        "to_addr": "Bob",
                                        "value": "10"                                         
                                    }]    
                                }
                        ] ,
                        "transaction2":[
                            {
                                    "inputs":[{                                                              
                                    },{
                                        "to_addr": "Alice",
                                        "value": "50"                           
                                    },{
                                        "to_addr": "Bob",
                                        "value": "10"                            
                                    }],    
                                    "outputs":[{       
                                        "to_addr": "Alice",
                                        "value": "48"                           
                                    },{
                                        "to_addr": "Bob",
                                        "value": "3"                                                          
                                    }]    
                                }
                        ]                             
                    }]                 
                }]     
             }] 
            }        
     ))
}