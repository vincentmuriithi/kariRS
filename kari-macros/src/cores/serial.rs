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

        #[cfg(feature = "esp")]
        let config = UartConfig::default().with_baudrate(#pin_num);
        #[cfg(feature = "esp")]
        let mut _kari_serial = Uart::new(_peripherals.UART0, config).unwrap()
        .with_rx(_peripherals.GPIO3)
        .with_tx(_peripherals.GPIO1);
        #[cfg(feature = "esp")]
        let (mut _kari_rx, mut _kari_serial) = _kari_serial.split();
        #[cfg(feature = "esp")]
        let mut kari_serial_listener = SerialListener::new(_kari_rx);
    };

    expanded.into()
}