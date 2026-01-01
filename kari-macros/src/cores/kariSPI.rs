use proc_macro2::Span;

use super::*;




pub fn expand_kariSPI(input: TokenStream) -> TokenStream{
    let args = parse_macro_input!(input with Punctuated<syn::Expr, Token![,]>::parse_terminated);

    if let Err(err) = check_num_of_pins(&args, 4) {
        return err;
    }

    for k in 0..args.len() {
        if let Err(err) = expect_int(&args[k], "Pin") {
            return  err;
        }
    }

    let clk = match expect_int(&args[0], "CLK") {
        Ok(value) => syn::Ident::new(&format!("d{}", value), Span::call_site()),
        Err(err) => return err
    };

    let mosi = match expect_int(&args[1], "MOSI") {
        Ok(value) => syn::Ident::new(&format!("d{}", value), Span::call_site()),
        Err(err) => return err
    };

    let miso = match expect_int(&args[2], "MISO") {
        Ok(value) => syn::Ident::new(&format!("d{}", value), Span::call_site()),
        Err(err) => return err
    };

    let cs =  match expect_int(&args[3], "CS") {
        Ok(value) => syn::Ident::new(&format!("d{}", value), Span::call_site()),
        Err(err) => return err
    };

    quote! {
        
        {
            let (mut spi, mut cs) = arduino_hal::spi::Spi::new( 
            dp.SPI, 
            kariPins.#clk.into_output(), 
            kariPins.#mosi.into_output(), 
            kariPins.#miso.into_pull_up_input(), 
            kariPins.#cs.into_output(),
            arduino_hal::spi::Settings::default(), 
            );

            kariSPI::new(spi, cs)
        }
    }
    .into()
}