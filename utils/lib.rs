pub mod stringify;
pub mod stepper;
pub mod calculate;
pub mod slicer;
pub mod stringtou128;

#[allow(unused_imports)]
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    static DIFFICULTY_STR:&str="0x00ffffffffffffffffffffffffffff00";    
    static DIFFICULTY_128:u128=0x00ffffffffffffffffffffffffffff00;        
    #[test]
    fn func_string_to_u128(){
         
        let difficulty=crate::stringtou128::string_to_u128(&DIFFICULTY_STR.to_string());
        assert_eq!(difficulty,0x00ffffffffffffffffffffffffffff00);
    }
    // #[test]
    // fn func_bytes_to_u128(){        
    //     let y=&u128_bytes(&difficulty_128);
    //     let x=difficulty_bytes_as_u128(&y.to_vec());
    //     assert_eq!(&x,&difficulty_128);
    // }
    // #[test]
    // fn func_bytes_to_u128(){
    //     let mut bytes:Vec<u8> = vec![];        
    //     bytes.extend(&u128_bytes(&difficulty_128));
    //     let x=difficulty_bytes_as_u128(&bytes).to_le_bytes().to_vec();
    //     assert_eq!(&x,&bytes);
    // }
    
    //#[test]
    //fn func_integration_string_vs_u128(){
        // let mut bytes = vec![];
        // let difficulty=string_to_u128(&difficulty_str.to_string());
        // let de_diff_bytes=u128_bytes(&difficulty);
        
        // bytes.extend(de_diff_bytes);
        
        // let de_diff = u128::from_le_bytes(de_diff_bytes);
        // let de_diff_str = format!("0x{de_diff:032x}");    
    
        // let aa=difficulty_bytes_as_u128(&bytes);        
        // assert_eq!(&difficulty_str, &de_diff_str);
        // assert_eq!(&difficulty, &de_diff);
        // assert_eq!(&de_diff, &aa);
    //}
}
