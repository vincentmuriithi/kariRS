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
        {
            let mut analog_value = 0;
            #[cfg(feature = "esp")]
            {
               let val =  if let Some(analog_pin) = #analog_pin_ident.as_mut() {
                    match analog_pin {
                        kariADCType::ADC1(pin) => {analog_value = nb::block!(_adc1.as_mut().unwrap().read_oneshot(pin)).unwrap();},
                        kariADCType::ADC2(pin) => {analog_value = nb::block!(_adc2.as_mut().unwrap().read_oneshot(pin)).unwrap();},
                    }
                } else {
                    analog_value = 0;
                };

            }

            #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
            unsafe {
                if let Some(analog_pin) = #analog_pin_ident.as_mut() {
                    analog_value = analog_pin.analog_read(&mut kari_adc);
                } else {
                    analog_value = 0;
                }
            }
            analog_value
        }
    };

    expanded.into()
}