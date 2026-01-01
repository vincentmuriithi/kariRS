use syn::{Ident, Token, parse::Parse};


#[allow(non_camel_case_types)]
pub struct pinDecl {
    pub name: Ident,
    pub _colon: Token![:],
    pub pin: Ident
}


impl Parse for pinDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(pinDecl { 
            name: input.parse()?,
             _colon: input.parse()?,
              pin: input.parse()? 
            })
    }
}

