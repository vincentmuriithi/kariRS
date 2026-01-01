#![allow(non_snake_case)]

pub use proc_macro::{TokenStream};
pub use quote::quote;
pub use syn::{Expr, Token, parse_macro_input, punctuated::{self, Punctuated}, token::Token};
// pub use crate::structs::*;
// pub use quote::ToTokens;
// pub use crate::func::*;

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod _Names {
    pub const _kariAnalogPin: &str = "_kariAnalogPin";
    pub const _kariPWMPin: &str = "_kariPWMPin";
    pub const _kariOutputPin: &str = "_kariOutputPin";
    pub const _kariInputPin: &str = "_kariInputPin";
}


pub mod pinMode;
pub mod digitalRead;
pub mod digitalWrite;
pub mod analogWrite;
pub mod analogRead;



//use super::*;
//pub fn expand_pinMode(input: TokenStream) -> TokenStream{}