use super::*;



pub fn expand_analogWrite(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let pin_lit = &args[0];
    let analog_value = &args[1];

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

    // let analog_num = if let Expr::Lit(lit) = analog_value {
    //     if let syn::Lit::Int(i) = &lit.lit {
    //         i.base10_parse::<u16>().unwrap()
    //     } else {
    //        return syn::Error::new_spanned(
    //             pin_lit,
    //             "pin must be integer"
    //         )
    //         .to_compile_error()
    //         .into();
    //     }
    // } else {
    //     return syn::Error::new_spanned(
    //             pin_lit,
    //             "pin must be a literal"
    //         )
    //         .to_compile_error()
    //         .into();
    // };

    let var_name = syn::Ident::new(&format!("{}{}", _Names::_kariPWMPin, pin_num), proc_macro2::Span::call_site());

    let expanded = quote! {
        #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
        unsafe {
            if let Some(pwm_pin) = #var_name.as_mut() {
                let max_duty = pwm_pin.max_duty_cycle();
                let duty = (#analog_value as u16 * max_duty as u16) / 255;
                let _ = pwm_pin.set_duty_cycle(duty as _);
            }
        }

        #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
        unsafe {
            
            match #var_name.as_mut() {
                Some(kariChannel::LS(channel)) => {
                    let max_duty = channel.max_duty_cycle();
                    let duty = ((#analog_value as u16 * max_duty as u16) + 127) / 255;
                    let _ = channel.set_duty_cycle(duty as _);
                },
                Some(kariChannel::HS(channel)) => {
                    let max_duty = channel.max_duty_cycle();
                    let duty = ((#analog_value as u16 * max_duty as u16) + 127) / 255;
                    let _ = channel.set_duty_cycle(duty as _);
                },
                None => {},
            }
        }
    };

    expanded.into()
}