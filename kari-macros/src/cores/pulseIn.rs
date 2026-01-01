use super::*;



pub fn expand_pulseIn(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let pin_lit = &args[0];
    let pin_mode = &args[1];
    let timeout = args.get(2);

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

    let timeout_token = if let Some(_timeout) = timeout {
        quote! {
            #_timeout
        }
    } else {
        quote! { 1000000 }
    };

      if let Expr::Path(path ) = pin_mode {
        let ident = &path.path.segments.last().unwrap().ident;

        if ident.to_string() != "_HIGH" && ident.to_string() != "_LOW" {
            return syn::Error::new_spanned(pin_mode, "pin mode must be _HIGH or _LOW")
            .to_compile_error()
            .into();
        }
    } else {
        return syn::Error::new_spanned(pin_mode, "pin mode must be _HIGH or _LOW")
            .to_compile_error()
            .into();
    }

    


    let var_name = syn::Ident::new(&format!("{}{}", _Names::_kariInputPin, pin_num), proc_macro2::Span::call_site());
    let _pin_name = syn::Ident::new(&format!("d{}", pin_num), proc_macro2::Span::call_site());

    quote! {
        unsafe {
               if let Some(pin) = #var_name.as_mut() {

                    let interval = #timeout_token;
                    let mut is_success = true;
                    let mut cascade = true;
                    let start = micros();

                    if #pin_mode == _HIGH {

                        while pin.is_high() {
                            if micros().wrapping_sub(start) >= interval { 
                                is_success = false;
                                break; 
                            }
                        }


                        while !pin.is_high() {
                            if micros().wrapping_sub(start) >= interval { 
                                is_success = false;
                                break; 
                            }
                        }

                        let pulse_start = micros();

                        while pin.is_high() {
                            if micros().wrapping_sub(start) >= interval { 
                                is_success = false;
                                break; 
                            }
                        }

                        //if is_success { micros().wrapping_sub(pulse_start) } else { 5 }
                        micros().wrapping_sub(pulse_start)

                    } else {
                        while !pin.is_high() {
                            if micros().wrapping_sub(start) >= interval { 
                                is_success = false;
                                break; 
                            }
                        }

                        while pin.is_high() {
                            if micros().wrapping_sub(start) >= interval { 
                                is_success = false;
                                break; 
                            }
                        }

                        let pulse_start = micros();

                        while !pin.is_high() {
                            if micros().wrapping_sub(start) >= interval { 
                                is_success = false;
                                break; 
                            }
                        }

                        if is_success { micros().wrapping_sub(pulse_start) } else { 5 }   
                    }
                } else {
                   0
                }

            }
    }.into()
}