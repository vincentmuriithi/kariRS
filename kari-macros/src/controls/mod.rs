#![allow(non_snake_case)]

pub use proc_macro::{TokenStream};
pub use quote::{format_ident, quote};
pub use syn::{Expr, ExprAssign, ExprClosure, Ident, ItemFn, LitInt, Token, parse::Parse, parse_macro_input, punctuated::{self, Punctuated}, token::Token};
pub use crate::structs::*;
pub use quote::ToTokens;
pub use crate::func::*;



pub mod kariAsync;
pub mod kariPulse;
pub mod kariSequential;
pub mod kariBegin;





//use super::*;
//pub fn expand_serial(input: TokenStream) -> TokenStream{}