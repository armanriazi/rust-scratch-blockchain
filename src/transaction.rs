use super::*;
use std::{collections::HashSet, clone};

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64
}
trait TraitOutPut{}
impl TraitOutPut for Output{}

/// Overspending = Where did the money come from?  inputs must be >= sum of values of generated outputs
/// </br></br>
/// Double Spending = Is the money avaliable? any one output is never used as an input more than once
/// </br></br>
/// Impersonation= Who owns the money and who is sending it?  Solved by adding signature and smart contract(not cover in this example)
/// </br></br>
/// Trx contain 2 pieces of info: Set of I/O that I=O - Value of TRXs=Sum(Inputs) Value of the Fee =Sum(Inputs)-Sum(Outputs)
/// </br></br>
/// We implement coinbase TRXs model: do not require inputs, produce an output - allow the miner to collect all the trx fees in that block and that block's block reward (coin genesis)

trait TraitClosureTransaction{
    fn new(&self, dyn FnMut(Vec<Output>) ->  Vec<Output>) -> Self where Self: Sized;
}
pub struct ClosureTransaction
{
    pub calculation: dyn FnMut(Vec<Output>) -> Vec<Output>,        
}

impl TraitClosureTransaction for ClosureTransaction{
    fn new(&self,calc) -> Self where Self: Sized {
        &self.calculation=calc;
    }
}

pub struct Transaction
{
    pub inputs: Option<Vec<Output>>,
    pub outputs: Option<Vec<Output>>,
}

impl TraitClosureTransaction for Transaction{
    fn new(&self) -> Self where Self: Sized {
        todo!()
    }
}
// macro_rules! call_on_self {
//     ($receiver:ident, $F:ident) => {
//         $receiver.$F()
//     };
// }
fn return_closure_hash_data(in_or_out: &str)-> impl Fn(&Vec<Output>)-> HashSet<Hash>
     {         
         |ref inputs| {
                    inputs
                    .iter()                
                    .map(|input| input.hash())
                    .collect::<HashSet<Hash>>()           
    }    
}
//fn vec_to_hashset(self:Vec<Output>) -> Box<dyn T> where T:HashSet<Hash>;


impl Transaction
{
  

    fn inputs(&mut self, arg: Vec<Output>) -> Vec<Output> {
        match self.inputs {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.inputs = Some(v);
                v
            }
        }
        
    }

    fn outputs(&mut self, arg: Vec<Output>) -> Vec<Output> {
        match self.outputs {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.outputs = Some(v);
                v
            }
        }        
    }
    
    pub fn input_value (&self) -> u64 {
        self.inputs.unwrap()
            .iter()
            .map(|input| input.value)
            .sum()
    }

    pub fn output_value (&self) -> u64 {
        self.outputs.unwrap()
            .iter()
            .map(|output| output.value)
            .sum()
    }

    pub fn input_hashes (&self) -> HashSet<Hash> {

        //let output=return_closure_hash_data("input");
        //output((&self.inputs))

        let mut c = ClosureTransaction::new(|a| a);

        let v1 = c.value(1);
    }

    pub fn output_hashes (&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase (&self) -> bool {
        self.inputs.unwrap().len() == 0
    }
}

#[test]
fn call_with_different_puts() {
    let mut c =Transaction::new(|a| a);
    
    let mut input: Vec<Output> = Vec::new();
    let inpt=Output{to_addr:"Add1",value:10};
    inpt.push(inpt);

    let mut output: Vec<Output> = Vec::new();
    let oupt=Output{to_addr:"Add2",value:20};    
    output.push(oupt);
    
    
    let v1 = c.inputs(input);
    let v2 = c.outputs(output);

    assert_eq!(v1, vec!["Add1",10]);
    assert_eq!(v2, vec!["Add2",20]);
}


impl Hashable for Output {
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
