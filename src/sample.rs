use serde_json::json;

use crate::CustomError;

pub fn sample_trx_json_data_from_module() -> Result<serde_json::Value, CustomError> {
    Ok(json!({
    "blocks":[{
        "block1":[{
            "transactions":[{
                "transaction1":[{
                        "inputs":[{
                           
                        }],
                        "outputs":[{
                            "to_addr": "Alice",
                            "value": "50"
                        },{
                            "to_addr": "Bob",
                            "value": "10"
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
                            "value": "50"
                        },{
                            "to_addr": "Bob",
                            "value": "10"
                        }]
                }] ,
                "transaction2":[{
                        "inputs":[{                    
                            "to_addr": "Alice",
                            "value": "49"
                        },{
                            "to_addr": "Bob",
                            "value": "9"
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
        "block3":[{
                "transactions":[{
                    "transaction1":[{
                            "inputs":[{
                                "to_addr": "Mina",
                                "value": "48"
                            },{
                                "to_addr": "Nuha",
                                "value": "3"
                            }],
                            "outputs":[{
                                "to_addr": "Tina",
                                "value": "10"
                            },{
                                "to_addr": "Bina",
                                "value": "10"
                            }]
                    }],
                    "transaction2":[{
                            "inputs":[{                            
                                "to_addr": "Tina",
                                "value": "10"
                            },{
                                "to_addr": "Bina",
                                "value": "10"
                            }],
                            "outputs":[{                             
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
                                "value": "0"                                                          
                            }]    
                    }]                             
                }]                 
        }]      
    }]"#;

    let js = serde_json::from_str(json).unwrap();

    Ok(js)
}
