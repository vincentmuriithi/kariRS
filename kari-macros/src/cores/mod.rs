#![allow(non_snake_case)]

pub use proc_macro::{TokenStream};
pub use quote::{quote};
pub use syn::{Expr, LitInt, Token, parse_macro_input, punctuated::{self, Punctuated}, token::Token};
pub use crate::structs::*;
pub use quote::ToTokens;
pub use crate::func::*;
pub use crate::gpio::_Names;


pub mod serial;
pub mod kprintln;
pub mod pulseIn;
pub mod eeprom;
pub mod kariSPI;











//use super::*;
//pub fn expand_serial(input: TokenStream) -> TokenStream{}