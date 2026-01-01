use super::*;



pub fn expand_digitalRead(input: TokenStream) -> TokenStream {
     let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let pin_lit = &args[0];

    let pin_num = if let Expr::Lit(lit) = pin_lit {
        if let syn::Lit::Int(i) = &lit.lit {
            i.base10_parse::<u8>().unwrap()
        } else {
           return syn::Error::new_spanned(
                pin_lit,
                "pin must be integer"
            )
            .to_compile_error()
            .into();
        }
    } else {
        return syn::Error::new_spanned(
                pin_lit,
                "pin must be a literal"
            )
            .to_compile_error()
            .into();
    };


    let var_name = syn::Ident::new(&format!("{}{}", _Names::_kariInputPin, pin_num), proc_macro2::Span::call_site());

    let expanded = quote! {
        if let Some(pin) = #var_name.as_mut() {
            if pin.is_high(){
                kari_hal::gpio::HIGH
            } else {
                kari_hal::gpio::LOW
            }
        } else {
            kari_hal::gpio::LOW
        }
    };

    expanded.into()

}