use super::*;



pub fn expand_digitalWrite(input: TokenStream) -> TokenStream {
     let args = parse_macro_input!(input  with Punctuated<Expr, Token![,]>::parse_terminated);
    let pin_lit = &args[0];
    let pin_mode = &args[1];

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
                "pin must be literal"
            )
            .to_compile_error()
            .into();
    };
    

    if let Expr::Path(path ) = pin_mode {
        let ident = &path.path.segments.last().unwrap().ident;

        if ident.to_string() != "HIGH" && ident.to_string() != "LOW" {
            return syn::Error::new_spanned(pin_mode, "pin mode must be HIGH or LOW")
            .to_compile_error()
            .into();
        }
    } else {
        return syn::Error::new_spanned(pin_mode, "pin mode must be HIGH or LOW")
            .to_compile_error()
            .into();
    }


    let var_name = syn::Ident::new(&format!("{}{}",_Names::_kariOutputPin, pin_num), proc_macro2::Span::call_site());
    let expanded = quote!{
        unsafe {
            if #pin_mode == kari_hal::gpio::HIGH {
                if let Some(pin) = #var_name.as_mut() {
                    pin.set_high();
                }
            } else if #pin_mode == kari_hal::gpio::LOW {
                if let Some(pin) = #var_name.as_mut() {
                    pin.set_low();
                }
            }
        }
    };

    expanded.into()
}