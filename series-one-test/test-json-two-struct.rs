    
    // #[derive(Debug,Clone,Deserialize,Serialize)]
    // pub enum Transactions {
    //      puts
    // }
    // #[derive(Debug,Clone,Deserialize,Serialize)]
    // pub enum Transaction {
    //     to_addr,
    //     value
    // }

    #[cfg(test)]
    mod tests {
        use std::fs;
        use serde_json::{json, Value,Result};
        use serde::*;
        use super::*;
    
        #[derive(Debug,Deserialize,PartialEq)]
        pub struct ModelValue {        
            to_addr:String,
            value:String        
        }
        #[derive(Debug,Deserialize,PartialEq)]
        pub struct Model {
            inputs:Vec<ModelValue>        
        }
        impl ToString for Model{
            fn to_string(&self)-> String{
               
                // let str = format!("{:?}",&self);
                // str.find("\"");
                // format!("{:?}",&self)
                //format!("{} {}",&self.to_addr,&self.value)
                 unimplemented!()
            }
        }
    
        #[test]
        fn sample_trx_json_data_block2_from_file(){
    
                let j = "
                {
                    \"inputs\":[{
                        \"to_addr\": \"Alice\",
                        \"value\": \"Ab\"                                       
                    }]   
                }";
                
            let m:Model = serde_json::from_str(j).unwrap();
            
            let modelvalue= ModelValue{            
                    to_addr: "Alice".to_string(),
                    value: "Ab".to_string()     
            };
            let mut vec_modelvalue=vec![];
            vec_modelvalue.push(modelvalue);
            let model= Model{            
              inputs:  vec_modelvalue
            };
           
            assert_eq!(model,m);
         }
    
    
    
         #[test]
         fn  untyped_example2() {
            // The type of `john` is `serde_json::Value`
            
            let data = json!({
                "inputs":[{
                  "to_addr": "Alice",
                  "value": "Ab"     
                }]
            });
            let data_inputs=(data.as_object().unwrap()).get("inputs").unwrap();  
            println!("Model : {} {}", data_inputs["to_addr"],data_inputs["value"]);
                    
            println!("{}", data.to_string());
        }
        #[test]
        fn untyped_example() -> Result<()> {
                // Some JSON input data as a &str. Maybe this comes from the user.
                let data = r#"
                    {
                        "inputs":[{
                            "to_addr": "Alice",
                            "value": "Ab"       
                        }]
                    }"#;
            
                let m: Model = serde_json::from_str(data)?;
                                
                println!("Model: {} {}", m.inputs[0].to_addr,m.inputs[0].value);
    
                let modelvalue= ModelValue{            
                    to_addr: "Alice".to_string(),
                    value: "Ab".to_string()     
                };
                let mut vec_modelvalue=vec![];
                vec_modelvalue.push(modelvalue);
                let model= Model{            
                     inputs:  vec_modelvalue
                }; 
                
                assert_eq!(m,model);
                
                Ok(())
         }
    }      
    
    //let u:serde_json::Value  = serde_json::from_str(&j).unwrap();
            //let u:User  = serde_json::from_str(&j).unwrap();
            //println!("{:#?}", i);  