
// macro_rules! call_on_self {
//     ($receiver:ident, $F:ident) => {
//         $receiver.$F()
//     };
// }
// fn return_closure_hash_data(in_or_out: &str)-> impl Fn(&Vec<Output>)-> HashSet<Hash>
//      {         
//          |ref inputs| {
//                     inputs
//                     .iter()                
//                     .map(|input| input.hash())
//                     .collect::<HashSet<Hash>>()           
//     }    
// }
use super::*;
use std::collections::HashSet;


#[derive(Debug,Clone)]
pub struct Value {
    pub to_addr: Address,
    pub value: u64,
}

// impl PartialEq<Value> for Value {
//     fn eq(&self, other: &Value) -> bool {
//         self.to_addr== other.to_addr && self.value== other.value 
//     }
// }

/// Overspending = Where did the money come from?  inputs must be >= sum of values of generated outputs
/// </br></br>
/// Double Spending = Is the money avaliable? any one output is never used as an input more than once
/// </br></br>
/// Impersonation= Who owns the money and who is sending it?  Solved by adding signature and smart contract(not cover in this example)
/// </br></br>
/// Trx contain 2 pieces of info: Set of I/O that I=O - Value of TRXs=Sum(Inputs) Value of the Fee =Sum(Inputs)-Sum(Outputs)
/// </br></br>
/// We implement coinbase TRXs model: do not require inputs, produce an output - allow the miner to collect all the trx fees in that block and that block's block reward (coin genesis)

#[derive(Debug)]
pub struct OptionTransaction {
    pub puts: Option<Transaction>
}

#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<Value>,
    pub outputs: Vec<Value>,
}
pub enum IO{
    Input,
    Output
}

//pub trait Put where Self: Sized {}
pub trait SuperTransaction {}


 pub trait Put where
    Self: SuperTransaction{
     fn returns_closure_io(&self,io: &IO) -> Box<(dyn Fn() -> u64 + '_)>;
     fn returns_closure_io_hash(&self,io: &IO) -> Box<(dyn Fn() -> HashSet<Hash> + '_)>;
}

impl SuperTransaction for Transaction {}
//impl Put for SuperTransaction {}

impl Put for Transaction {

    fn returns_closure_io(&self,io: &IO) -> Box<(dyn Fn() -> u64 + '_)> {
        match io {
         IO::Input => {Box::new(|| {
                self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
            })   
         }  
         IO::Output => {Box::new(|| {
            self.outputs
                 .iter()
                .map(|output| output.value)
                .sum()
            })
          }
        }  
    }

    fn returns_closure_io_hash(&self,io: &IO) -> Box<(dyn Fn() -> HashSet<Hash> + '_)> {
        match io {
         IO::Input => {Box::new(|| {
            self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
            })   
         }  
         IO::Output => {Box::new(|| {
            self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
            })
          }
        }  
    }
}
  
impl Transaction {

    pub fn default() -> OptionTransaction {        
        Self::new(vec![], vec![
            transaction::Value {
                to_addr: "Alice".to_owned(),
                value: 50,
            },
            transaction::Value {
                to_addr: "Bob".to_owned(),
                value: 50,
            },
        ])
    }

   pub fn new(inputs: Vec<Value>, outputs: Vec<Value>) ->  OptionTransaction {       
       
       OptionTransaction{ puts:
       Some(Self {            
            inputs,
            outputs,
        })}
    }

   pub fn trx_data<F>(&mut self, mut f: F) // We bring in self, but only f is generic F. f is the closure    
    where
        F: FnMut(&mut Vec<Value>, &mut Vec<Value>), // The closure takes mutable vectors of u32
                                                // which are the year and population data
    {
        f(&mut self.inputs, &mut self.outputs) // Finally this is the actual function. It says
                                                  // "use a closure on self.years and self.populations"
                                                  // We can do whatever we want with the closure
    }



    pub fn is_coinbase (&self) -> bool {       
        self.inputs.len() == 0
    }
}

impl Hashable for Value {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
     }
}

impl Hashable for Transaction {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}
