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
        #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
        let mut _kari_serial = arduino_hal::default_serial!(dp, kariPins, #pin_num);
        #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
        let (mut _kari_rx, mut _kari_serial) = _kari_serial.split(); //_kari_serial is just now the tx not the full object
       #[cfg(any(feature = "uno", feature = "mega", feature = "nano", feature = "leonardo"))]
        let mut kari_serial_listener = SerialListener::new(&mut _kari_rx);

        #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
        let config = UartConfig::default().with_baudrate(#pin_num);
        #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2"))]
        let mut _kari_serial = Uart::new(_peripherals.UART0, config).unwrap()
        .with_rx(_peripherals.GPIO3)
        .with_tx(_peripherals.GPIO1);
        #[cfg(feature = "esp32c3")]
        let mut _kari_serial = Uart::new(_peripherals.UART0, config).unwrap()
        .with_rx(_peripherals.GPIO21)
        .with_tx(_peripherals.GPIO20);
        #[cfg(feature = "esp32c6")]
        let mut _kari_serial = Uart::new(_peripherals.UART0, config).unwrap()
        .with_rx(_peripherals.GPIO17)
        .with_tx(_peripherals.GPIO16);
        #[cfg(feature = "esp32s3")]
        let mut _kari_serial = Uart::new(_peripherals.UART0, config).unwrap()
        .with_rx(_peripherals.GPIO44)
        .with_tx(_peripherals.GPIO43);
        #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
        let (mut _kari_rx, mut _kari_serial) = _kari_serial.split();
        #[cfg(any(feature = "esp", feature = "esp32", feature = "esp32s2", feature = "esp32s3", feature = "esp32c3", feature = "esp32c6"))]
        let mut kari_serial_listener = SerialListener::new(_kari_rx);
    };

    expanded.into()
}