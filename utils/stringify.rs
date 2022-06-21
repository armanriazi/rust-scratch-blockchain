#[allow(unused_macros)]
/// Convret &strt to String and vice versa
macro_rules! string {
    ($x:expr) => ( // <1>
        String::from(stringify!($x)) // <2>
    )
}