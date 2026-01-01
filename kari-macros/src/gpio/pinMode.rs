use super::*;


pub fn expand_pinMode(input: TokenStream) -> TokenStream{
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
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
        panic!("pin must be literal");
    };


    if let Expr::Path(path ) = pin_mode {
        let ident = &path.path.segments.last().unwrap().ident;

        if ident.to_string() != "OUTPUT" && ident.to_string() != "INPUT" && 
            ident.to_string() != "PWM" && ident.to_string() != "_INPUT" {
            return syn::Error::new_spanned(pin_mode, "pin mode must be OUTPUT or INPUT")
            .to_compile_error()
            .into();
        }
    } else {
         return syn::Error::new_spanned(pin_mode, "pin mode must be OUTPUT or INPUT")
            .to_compile_error()
            .into();
    }

  //  let pin_index = syn::
    let pin_ident = syn::Ident::new(&format!("d{}", pin_num), proc_macro2::Span::call_site());
    let analog_pin_ident = syn::Ident::new(&format!("a{}", pin_num), proc_macro2::Span::call_site());
    let var_name = syn::Ident::new(&format!("{}{}", _Names::_kariOutputPin, pin_num), proc_macro2::Span::call_site());
    let input_name = syn::Ident::new(&format!("{}{}",_Names::_kariInputPin, pin_num), proc_macro2::Span::call_site());
    let pwm_pin = syn::Ident::new(&format!("{}{}",_Names::_kariPWMPin, pin_num), proc_macro2::Span::call_site());
    let analog_pin = syn::Ident::new(&format!("{}{}", _Names::_kariAnalogPin, pin_num), proc_macro2::Span::call_site());
    let expanded = if let Expr::Path(path) = pin_mode {
        let ident = &path.path.segments.last().unwrap().ident;
        let _pin_num_lit = syn::LitInt::new(&pin_num.to_string(), proc_macro2::Span::call_site());

        
        let pwm_match = match pin_num {
            11 | 12 | 13 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer1);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },
            2 | 3 | 5 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer3);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },
            6 | 7 | 8 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer4);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },
            4 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer0);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },
            _ => quote! {
                compile_error!("Unsupported pin for PWM");
            }
        };


        
        let pwm_match_uno = match pin_num {
            9 | 10 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer1);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },
            5 | 6 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer0);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },
            3 | 11 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer2);
                pwm_object.enable();
                #pwm_pin = Some(pwm_object);
            },

            _ => quote! {
                compile_error!("Unsupported pin for PWM");
            }
        };

        match ident.to_string().as_str() {
            "OUTPUT" => quote! {
                let mut #var_name = None;
                unsafe {
                    if #pin_mode == kari_hal::gpio::OUTPUT{
                        #var_name = Some(kariPins.#pin_ident.into_output().downgrade());
                    } 
                }
            },

            "INPUT" => quote! {
                let mut #input_name = None;
                unsafe {
                    if #pin_mode == kari_hal::gpio::INPUT{
                        #input_name = Some(kariPins.#pin_ident.into_pull_up_input().downgrade());
                    } 
                }
            },

            "_INPUT" => quote! {
                let mut #analog_pin = None;
                unsafe {
                    if #pin_mode == kari_hal::gpio::_INPUT{
                        #analog_pin = Some(kariPins.#analog_pin_ident.into_analog_input(&mut kari_adc));
                    } 
                }
            },

            "PWM" => quote! {
                let mut #pwm_pin = None;
                unsafe {
                    if #pin_mode == kari_hal::gpio::PWM{

                        #[cfg(feature= "uno")]
                        unsafe {
                            #pwm_match_uno
                        }

                        #[cfg(feature = "mega")]
                        unsafe {
                            #pwm_match
                        }

                    }
                }
            },
           
            _ => quote! {compile_error!("Invalid Pin Mode, Modes supported are [INPUT, OUTPUT, _INPUT, PWM] only")},
        }
    } else {
       quote! {compile_error!("Invalid pin Mode");}
    };  // expanded
    

    expanded.into()
}