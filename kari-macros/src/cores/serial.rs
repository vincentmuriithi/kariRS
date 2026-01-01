use super::*;



pub fn expand_serial(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let pin_lit = &args[0];

    let pin_num = if let Expr::Lit(lit) = pin_lit {
        if let syn::Lit::Int(num) = &lit.lit {
            num.base10_parse::<u32>().unwrap()
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

    let expanded = quote! {
        let mut _kari_serial = arduino_hal::default_serial!(dp, kariPins, #pin_num);
        let (mut _kari_rx, mut _kari_serial) = _kari_serial.split(); //_kari_serial is just now the tx not the full object
       
        let mut kari_serial_listener = SerialListener::new(&mut _kari_rx);
    };

    expanded.into()
}