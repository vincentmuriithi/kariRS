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
            ident.to_string() != "INPUT_PULLUP" && ident.to_string() != "INPUT_PULLDOWN" 
            && ident.to_string() != "PWM" && ident.to_string() != "_INPUT" {
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
    let esp_pin_ident = format_ident!("GPIO{}", pin_num);
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
            9 | 10 => quote! {
            let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer2);
            pwm_object.enable();
            #pwm_pin = Some(pwm_object);
            },
            44 | 45 | 46 => quote! {
                let mut pwm_object = kariPins.#pin_ident.into_output().into_pwm(&mut _kari_timer5);
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
                #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::OUTPUT{
                        #var_name = Some(kariPins.#pin_ident.into_output().downgrade());
                    } 
                }

                #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::OUTPUT{
                        #var_name = Some(Output::new(_peripherals.#esp_pin_ident, Level::Low, OutputConfig::default()));
                    } 
                }
            },

            "INPUT" => quote! {
                let mut #input_name = None;
                #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::INPUT{
                        #input_name = Some(kariPins.#pin_ident.into_pull_up_input().downgrade());
                    } 
                }

                #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::INPUT{
                        #input_name = Some(Input::new(_peripherals.#esp_pin_ident, InputConfig::default()));
                    } 
                }
            },

            "INPUT_PULLUP" => quote! {
                let mut #input_name = None;

                #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::INPUT{
                        #input_name = Some(kariPins.#pin_ident.into_pull_up_input().downgrade());
                    } 
                }

                #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::INPUT{
                        #input_name = Some(Input::new(_peripherals.#esp_pin_ident, InputConfig::default().with_pull(Pull::Up)));
                    } 
                }
            },

            "INPUT_PULLDOWN" => quote! {
                let mut #input_name = None;

                #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
                compile_error!("INPUT_PULLDOWN is not supported on AVR chips. Use an external resistor instead.");

                #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::INPUT{
                        #input_name = Some(Input::new(_peripherals.#esp_pin_ident, InputConfig::default().with_pull(Pull::Down)));
                    } 
                }
            },

            "_INPUT" => quote! {
                
                let mut #analog_pin = None;
                
                #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::_INPUT{
                        #analog_pin = Some(kariPins.#analog_pin_ident.into_analog_input(&mut kari_adc));
                    } 
                }

                #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::_INPUT{
                        match #pin_num {
                            #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s3", feature = "esp32s2"))]
                            32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 => {
                                //#analog_pin = Some(kariAnyADC::ADC1(Adc::new(_peripherals.ADC1, adc_config)));
                                is_any_adc1_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC1(_adc1_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)));
                            }

                            #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s3", feature = "esp32s2"))]
                            0 | 2 | 4 | 12 | 13 | 14 | 15 | 25 | 26 | 27 => {
                                //#analog_pin = Some(kariAnyADC::ADC2(Adc::new(_peripherals.ADC2, adc_config)));
                                is_any_adc2_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC2(_adc2_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)));
                            }

                            #[cfg(feature = "esp32c6")]
                            0 | 2 | 3 | 4 | 5 | 6 => {
                                is_any_adc1_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC1(
                                    _adc1_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)
                                ));
                            }

                            #[cfg(feature = "esp32c3")]
                            0 | 2 | 3 | 4 => {
                                is_any_adc1_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC1(_adc1_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)));
                            }

                            #[cfg(feature = "esp32c3")]
                            5 => {
                                //#analog_pin = Some(kariAnyADC::ADC2(Adc::new(_peripherals.ADC2, adc_config)));
                                is_any_adc2_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC2(_adc2_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)));
                            }

                            #[cfg(any(feature = "esp32s3", feature = "esp32s2"))]
                            1..=10 => {
                                is_any_adc1_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC1(_adc1_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)));
                            }

                            #[cfg(any(feature = "esp32s3", feature = "esp32s2"))]
                            11..=20 => {
                                //#analog_pin = Some(kariAnyADC::ADC2(Adc::new(_peripherals.ADC2, adc_config)));
                                is_any_adc2_pin_init = true;
                                #analog_pin = Some(kariADCType::ADC2(_adc2_config.enable_pin(_peripherals.#esp_pin_ident, Attenuation::_11dB)));
                            }
                            _ => {
                               panic!("Pin does not support ADC");
                            }
                        }
                    } 
                }
            },

            "PWM" => quote! {
                // #[cfg(feature = "mega")]
                let mut #pwm_pin = None;

                // #[cfg(feature = "nano")]
                // let mut #pwm_pin: Option<avr_hal_generic::pwm::PwmPin<avr_hal_generic::hal::Atmega328p::TC2>> = None;
                #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
                unsafe {
                    if #pin_mode == kari_hal::gpio::PWM{

                        #[cfg(any(feature= "uno", feature= "nano"))]
                        unsafe {
                            #pwm_match_uno
                        }

                        #[cfg(feature = "mega")]
                        unsafe {
                            #pwm_match
                        }

                    }
                }

                #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
                {

                    if #pin_mode == kari_hal::gpio::PWM {
                        ls_timers.get_or_init(|| Vec::new());
                        let ls_timer_len = ls_timers.get().unwrap().len();
                       
                        if ls_timer_len < 8 {
                            let (channel_num, channel_str) = map_channel(ls_timer_len as usize);
                            let mut channel = configure_pwm(&mut _kari_ledc, channel_num, _peripherals.#esp_pin_ident, &_kari_lstimer0);
                            #pwm_pin = Some(kariChannel::LS(channel));
                            ls_timers.get_mut().unwrap().push(channel_str);
                        } 
                        #[cfg(any(feature = "esp", feature = "esp32"))]
                        else {
                            hs_timers.get_or_init(|| Vec::new());
                            let hs_timer_len = hs_timers.get().unwrap().len();
                            let (channel_num, channel_str) = map_channel(hs_timer_len as usize);
                            let mut channel = configure_pwm(&mut _kari_ledc, channel_num, _peripherals.#esp_pin_ident, &_kari_hstimer0);
                            #pwm_pin = Some(kariChannel::HS(channel));
                            hs_timers.get_mut().unwrap().push(channel_str);
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