use std::fmt::{Debug, Display};

pub mod asm_token;
pub mod rin_token;

pub trait Token: Display + Debug + Clone + PartialEq {
    fn from_string(string: String) -> Self;
    fn from_u64(num: u64) -> Self;
}
