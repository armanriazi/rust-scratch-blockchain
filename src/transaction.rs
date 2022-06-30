

use serde::{Deserialize, Serialize};

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
use std::{collections::HashSet, ops::Deref};

extern crate serde;

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Amount {
    pub to_addr: Address,
    pub amount: u64,
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


// trait TraitOptionTransaction{}
// impl TraitOptionTransaction for OptionTransaction<'_>{}

//#[derive(Serialize, Deserialize)]
//#[serde(remote = "OptionTransaction")]

//#[derive(Debug,Serialize, Deserialize)]
// pub struct OptionTransaction<'a>{
//     pub puts:  Option<&'a Transaction>
// }
// impl<'a> OptionTransaction<'a>{
//     pub fn new(inputs: Vec<Amount>, outputs: Vec<Amount>) ->  Self {       
               
//         Some(Self {            
//              inputs,
//              outputs,
//          })
//      }
    // pub fn new(&self) -> OptionTransaction<'a> {
    //     let puts = self.puts;
    //     OptionTransaction { puts, }
    // }
//}

// impl<'a> std::ops::DerefMut for OptionTransaction<'a> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         mut self.puts
//     }
// }
// impl<'a> Default for OptionTransaction<'a>{
    
//     fn default () -> Self {
//         OptionTransaction {
//             puts: &None
//         }
//     }
// }
// impl<'a> Deref for OptionTransaction<'a> {    
//     type Target = Option<&'a Transaction>;
//     fn deref(&self) -> &Self::Target {        
//     self.puts.as_ref()
//     }
// }
#[derive(Debug,Deserialize,Serialize)]
pub struct Transaction {
    pub inputs: Vec<Amount>,
    pub outputs: Vec<Amount>,
}

pub enum IO{
    Input,
    Output
}
pub enum IOH{
    Input,
    Output
}

//pub trait Put where Self: Sized {}
pub trait SuperTransaction {}


 pub trait Put where
    Self: SuperTransaction{
     fn returns_closure_io(&self,io: &IO) -> Box<(dyn Fn() -> u64 + '_)>;
     fn returns_closure_io_hash(&self,io: &IOH) -> Box<(dyn Fn() -> HashSet<Hash> + '_)>;
}

impl<'a> SuperTransaction for Transaction {}
//impl Put for SuperTransaction {}

impl Put for Transaction {

    fn returns_closure_io(&self,io: &IO) -> Box<(dyn Fn() -> u64 + '_)> {
        match io {
         IO::Input => {Box::new(|| {
                self.inputs
            .iter()
            .map(|input| input.amount)
            .sum()
            })   
         }  
         IO::Output => {Box::new(|| {
            self.outputs
                 .iter()
                .map(|output| output.amount)
                .sum()
            })
          }
        }  
    }

    fn returns_closure_io_hash(&self,io: &IOH) -> Box<(dyn Fn() -> HashSet<Hash> + '_)> {
        match io {
            IOH::Input => {Box::new(|| {
            self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
            })   
         }  
         IOH::Output => {Box::new(|| {
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

    // pub fn default() -> OptionTransaction<'_> {        
    //     Self::new(vec![         
                           
    //     ], vec![
    //         // transaction::Value {
    //         //     to_addr: "Alice".to_owned(),
    //         //     value: 47,
    //         // },
    //         // transaction::Value {
    //         //     to_addr: "Bob".to_owned(),
    //         //     value: 3
    //         // }, 
    //    ])
    // }
    pub fn new(inputs: Vec<Amount>, outputs: Vec<Amount>) ->  Transaction {                      
         Self{            
            inputs:inputs,
            outputs:outputs,
         }
     }

//    pub fn trx_data<F>(&mut self, mut f: F) // We bring in self, but only f is generic F. f is the closure    
//     where
//         F: FnMut(&mut Vec<Value>, &mut Vec<Value>), // The closure takes mutable vectors of u32
//                                                 // which are the year and population data
//     {
//         f(&mut self.inputs, &mut self.outputs) // Finally this is the actual function. It says
//                                                   // "use a closure on self.years and self.populations"
//                                                   // We can do whatever we want with the closure
//     }

    pub fn is_coinbase (&self) -> bool {               
     (&self.inputs).len() as u8 == 0
    }
}

impl Hashable for Amount{
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.amount));

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
