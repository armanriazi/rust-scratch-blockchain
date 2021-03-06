use library_blockchain::CustomError;
use serde_json::json;

pub fn sample_trx_json_data_from_module() -> Result<serde_json::Value, CustomError> {
    Ok(json!({
    "blocks":[{
        "block1":[{
            "transactions":[{
                "transaction1":[{
                        "inputs":[{
                            "to_addr": "",
                            "value": ""
                        }],
                        "outputs":[{
                            "to_addr": "Alice",
                            "value": "50"
                        },{
                            "to_addr": "Bob",
                            "value": "10"
                        }]
                }] ,
                "transaction2":[{
                        "inputs":[{
                        },{
                            "to_addr": "Alice",
                            "value": "50"
                        },{
                            "to_addr": "Bob",
                            "value": "10"
                        }],
                        "outputs":[{
                            "to_addr": "Mina",
                            "value": "48"
                        },{
                            "to_addr": "Nuha",
                            "value": "3"
                        }]
                }]
            }]
        }],
        "block2":[{
                "transactions":[{
                    "transaction1":[{
                            "inputs":[{
                            }],
                            "outputs":[{
                                "to_addr": "Alice",
                                "value": "10"
                            },{
                                "to_addr": "Bob",
                                "value": "0"
                            }]
                    }],
                    "transaction2":[{
                            "inputs":[{
                            },{
                                "to_addr": "Alice",
                                "value": "10"
                            },{
                                "to_addr": "Bob",
                                "value": "0"
                            }],
                            "outputs":[{
                                "to_addr": "Mina",
                                "value": "0"
                            },{
                                "to_addr": "Nuha",
                                "value": "232"
                            }]
                    }]
                }]
        }]
      }]
    }))
}

pub fn sample_trx_json_data_from_string() -> Result<serde_json::Value, CustomError> {
    let json = r#"
    "blocks":[{    
        "block1":[{
            "transactions":[{
                "transaction1":[{
                        "inputs":[{        
                            "to_addr": "",
                            "value": ""                               
                        }],    
                        "outputs":[{
                            "to_addr": "Alice",
                            "value": "50"                           
                        },{
                            "to_addr": "Bob",
                            "value": "10"                                         
                        }]    
                }] ,
                "transaction2":[{
                        "inputs":[{                                                              
                        },{
                            "to_addr": "Alice",
                            "value": "50"                           
                        },{
                            "to_addr": "Bob",
                            "value": "10"                            
                        }],    
                        "outputs":[{       
                            "to_addr": "Mina",
                            "value": "48"                           
                        },{
                            "to_addr": "Nuha",
                            "value": "3"                                                          
                        }]    
                }]                             
            }]
        }],
        "block2":[{
                "transactions":[{
                    "transaction1":[{
                            "inputs":[{                           
                            }],    
                            "outputs":[{
                                "to_addr": "Alice",
                                "value": "10"                           
                            },{
                                "to_addr": "Bob",
                                "value": "0"                                         
                            }]    
                    }],
                    "transaction2":[{
                            "inputs":[{                                                              
                            },{
                                "to_addr": "Alice",
                                "value": "10"                           
                            },{
                                "to_addr": "Bob",
                                "value": "0"                            
                            }],    
                            "outputs":[{       
                                "to_addr": "Mina",
                                "value": "0"                           
                            },{
                                "to_addr": "Nuha",
                                "value": "232"                                                          
                            }]    
                    }]                             
                }]                 
        }]      
    }]"#;

    let js = serde_json::from_str(json).unwrap();

    Ok(js)
}
