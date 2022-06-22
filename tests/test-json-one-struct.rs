    
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
    pub struct Model {
        to_addr:String,
        value:String
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
                \"to_addr\": \"Alice\",
                \"value\": \"Ab\"                                       
                    
            }";
            
        let m:Model = serde_json::from_str(j).unwrap();
        
        let i= Model{            
                to_addr: "Alice".to_string(),
                value: "Ab".to_string()     
        };
       
        assert_eq!(i,m);
     }



     #[test]
     fn  untyped_example2() {
        // The type of `john` is `serde_json::Value`
        let data = json!({
            "to_addr": "Alice",
            "value": "Ab"     
        });
    
        println!("Model : {} {}", data["to_addr"],data["value"]);
    
        // Convert to a string of JSON and print it out
        println!("{}", data.to_string());

        // let i= Model{            
        //     to_addr: "Alice".to_string(),
        //     value: "Ab".to_string()     
        // };       
        // println!("{:?}",data.to_string());
        //println!("{:?}",i.to_string());
        //assert_eq!(i.to_string(),data.to_string());
    }
    #[test]
    fn untyped_example() -> Result<()> {
            // Some JSON input data as a &str. Maybe this comes from the user.
            let data = r#"
                {
                    "to_addr": "Alice",
                    "value": "Ab"       
                }"#;
        
            // Parse the string of data into serde_json::Value.
            let m: Model = serde_json::from_str(data)?;
        
            // Access parts of the data by indexing with square brackets.
            println!("Model: {} {}", m.to_addr,m.value);

            let i= Model{            
                to_addr: "Alice".to_string(),
                value: "Ab".to_string()     
            };       
            
            assert_eq!(i,m);
            
            Ok(())
     }
}      

//let u:serde_json::Value  = serde_json::from_str(&j).unwrap();
        //let u:User  = serde_json::from_str(&j).unwrap();
        //println!("{:#?}", i);  