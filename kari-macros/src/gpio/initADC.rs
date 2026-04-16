use super::*;


pub fn expand_initADC(_input: TokenStream) -> TokenStream {
    quote! {
        #[cfg(any(feature= "uno", feature= "nano", feature= "mega", feature= "leonardo"))]
        {}


        #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
        {
             if is_any_adc1_pin_init {
                _adc1 = Some(Adc::new(_peripherals.ADC1, _adc1_config));
            }

            #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s3", feature = "esp32s2", feature = "esp32c3"))]
            // code to be run once for ADC2
            if is_any_adc2_pin_init {
                _adc2 = Some(Adc::new(_peripherals.ADC2, _adc2_config));
            }
        }
    }.into()
}