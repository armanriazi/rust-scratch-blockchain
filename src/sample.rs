use serde_json::{json};

use crate::CustomError;

pub fn sample_trx_json_data_block2() -> Result<serde_json::Value,CustomError>{
    
     Ok(json!({
        "transactions":[{
                "transaction1":[
                    {
                        "inputs":[{                  
                        }],
                        "outputs":[{
                            "to_addr": "Alex",
                            "value": "0",                            
                        },{
                            "to_addr": "Alice",
                            "value": "47",                            
                        },{
                            "to_addr": "Bob",
                            "value": "3",                            
                        }]    

                       
                    }
                ],              
                "transaction2":[
                    {
                       "inputs":[{                                                               
                        }],
                        "outputs":[{
                            "to_addr": "Alice",
                            "value": "0",
                        },{
                            "to_addr": "Nashu",
                            "value": "0",
                        }]
                    }
                ],              
                "transaction3":[
                    {
                       "inputs":[{                                                               
                        }],
                        "outputs":[{
                            
                        }]
                    }
                ]
            }]
        }))
}