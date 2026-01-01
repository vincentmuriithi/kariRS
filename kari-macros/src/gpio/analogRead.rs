use super::*;


pub fn expand_analogRead(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let pin_lit = &args[0];

    let pin_num = if let Expr::Lit(lit) = pin_lit {
        if let syn::Lit::Int(num) = &lit.lit {
            num.base10_parse::<u16>().unwrap()
        } else {
            return syn::Error::new_spanned(pin_lit, "pin must be integer")
            .to_compile_error()
            .into()
        }
    } else {
        return syn::Error::new_spanned(pin_lit, "pin must be a literal")
        .to_compile_error()
        .into()
    };

    let analog_pin_ident = syn::Ident::new(&format!("{}{}", _Names::_kariAnalogPin, pin_num), proc_macro2::Span::call_site());
    let expanded = quote! {
        unsafe {
            if let Some(analog_pin) = #analog_pin_ident.as_mut() {
                analog_pin.analog_read(&mut kari_adc)
            } else {
                0
            }
        }
    };

    expanded.into()
}