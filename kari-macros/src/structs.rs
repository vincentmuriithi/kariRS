use syn::{Ident, Token, parse::Parse};


#[allow(non_camel_case_types)]
pub struct pinDecl {
    pub name: Ident,
    pub _colon: Token![:],
    pub pin: Ident,
    pub mode: Option<Ident>
}


impl Parse for pinDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon = input.parse()?;
        let pin = input.parse()?;

        let mode = if input.peek(Ident) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(pinDecl { 
            name, 
             _colon,
              pin,
              mode
            })
    }
}

