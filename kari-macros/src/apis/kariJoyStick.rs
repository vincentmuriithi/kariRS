use super::*;



pub fn expand_kariJoyStick(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);

    if args.len() != 4 {
        return syn::Error::new_spanned(quote! { #args}, 
           "Expected 4 arguments: vrx, vry, sensitivity, threshold")
           .to_compile_error()
           .into()
    }

    let vrx = &args[0];
    let vry = &args[1];
    let sensitivity = &args[2];
    let threshold = &args[3];
    let mut vrx_num_ident: Option<Ident> = None;
    let mut vry_num_ident: Option<Ident> = None;

    for (name, expr) in [("vrx", vrx), ("vry", vry)] {
        if !matches!(expr, Expr::Lit(syn::ExprLit {lit: syn::Lit::Int(_), ..})){
            return syn::Error::new_spanned(expr, format!("{} must be an integer literal", name))
            .to_compile_error()
            .into();
        } else {
            if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(k), ..}) = expr {
                let num = k.base10_parse::<u8>().unwrap();
                let ident = syn::Ident::new(&format!("a{}", num), proc_macro2::Span::call_site());
                if name == "vrx" {
                    vrx_num_ident = Some(ident);
                } else {
                    vry_num_ident = Some(ident);
                }
            }
        }
    }


    let vrx_num_ident = vrx_num_ident.unwrap();
    let vry_num_ident = vry_num_ident.unwrap();
    
    
    let expanded = quote! {
       unsafe {
        let dp = arduino_hal::Peripherals::steal();
        let pins = arduino_hal::pins!(dp);
        let mut _adc = arduino_hal::Adc::new(dp.ADC, AdcSettings {
                clock_divider: ClockDivider::Factor128,
                ref_voltage: ReferenceVoltage::AVcc
        });

         kariJoyStick::new(
            || {
                unsafe {
                    let dp = arduino_hal::Peripherals::steal();
                    let pins = arduino_hal::pins!(dp);
                    let mut _adc = arduino_hal::Adc::new(dp.ADC, AdcSettings {
                            clock_divider: ClockDivider::Factor128,
                            ref_voltage: ReferenceVoltage::AVcc
                    });
                    let analog_pin = pins.#vrx_num_ident.into_analog_input(&mut _adc);
                    analog_pin.analog_read(&mut _adc)
                }
            },
            || {
                unsafe {
                    let dp = arduino_hal::Peripherals::steal();
                    let pins = arduino_hal::pins!(dp);
                    let mut _adc = arduino_hal::Adc::new(dp.ADC, AdcSettings {
                            clock_divider: ClockDivider::Factor128,
                            ref_voltage: ReferenceVoltage::AVcc
                    });
                    let analog_pin = pins.#vry_num_ident.into_analog_input(&mut _adc);
                    analog_pin.analog_read(&mut _adc)
                }
            },
            #sensitivity,
            #threshold
        )
       }
    };

    expanded.into()
}