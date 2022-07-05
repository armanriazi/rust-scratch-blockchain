#[allow(unused_imports)]
use std::ops::*;

pub fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i.add(j)
}
pub fn sub<T: std::ops::Sub<Output = T>>(i: T, j: T) -> T {
    i.sub(j)
}
// fn equal<T: std::ops::PartialEq<Output = bool>>(i: T, j: T) -> bool{
//     i.eq(&j)

// }

// impl PartialEq<T,U> for T {

// }
