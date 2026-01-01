use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, punctuated::Punctuated};


pub fn check_num_of_pins(args: &Punctuated<Expr, syn::token::Comma>, length: usize) -> Result<(), TokenStream> {
    if args.len() != length {
        return Err (syn::Error::new_spanned(quote! {#args}, 
           format!("Expected {} arguments but found {} arguments", length, args.len()))
           .to_compile_error()
           .into()
        )
    }

    Ok(())
}

pub fn expect_int(expr: &Expr, name: &str) -> Result<u8, TokenStream>{
    if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit),..}) = expr {
        lit.base10_parse::<u8>().map_err(|_|{
            syn::Error::new_spanned(expr,format!("{} must be an integer literal between 0 - 255", name))
            .to_compile_error()
            .into()
        })
    } else {
        Err(syn::Error::new_spanned(expr,format!("{} must be an integer literal", name))
            .to_compile_error()
            .into()
        )
    }
}


pub fn generate_analog_write_arms(pin: &syn::LitInt) -> Vec<proc_macro2::TokenStream> {
    let arms: Vec<_> = (0..=255).map(|value| {
        let pattern = syn::LitInt::new(&format!("{}u8", value), proc_macro2::Span::call_site());
        let lit = syn::LitInt::new(&format!("{}u16", value), proc_macro2::Span::call_site());
        quote! { #pattern => analogWrite!(#pin, #lit), }
    }).collect();
    arms
}