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

    let analog_num = if let Expr::Lit(lit) = analog_value {
        if let syn::Lit::Int(i) = &lit.lit {
            i.base10_parse::<u16>().unwrap()
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

    let var_name = syn::Ident::new(&format!("{}{}", _Names::_kariPWMPin, pin_num), proc_macro2::Span::call_site());

    let expanded = quote! {
        unsafe {
            if let Some(pwm_pin) = #var_name.as_mut() {
                let max_duty = pwm_pin.max_duty_cycle();
                let duty = (#analog_num * max_duty) / 255;
                pwm_pin.set_duty_cycle(duty);

            }
        }
    };

    expanded.into()
}