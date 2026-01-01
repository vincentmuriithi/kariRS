use super::*;




pub fn expand_kariAsync(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let closure = match syn::parse2::<ExprClosure>(args[0].to_token_stream()) {
        Ok(c) => c,
        Err(err) => return err.to_compile_error().into()
    };

    let pin_lit = match syn::parse2::<LitInt>(args[1].to_token_stream()) {
        Ok(num) => num,
        Err(err) => return err.to_compile_error().into()
    };

    let schedule_name = match syn::parse2::<Ident>(args[2].to_token_stream()) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error().into()
    };

    let schedule_name = schedule_name.to_string().to_uppercase();

    let pin_ident = syn::Ident::new(&format!("_{}", pin_lit), proc_macro2::Span::call_site());
    let previous = format_ident!("{}_PREVIOUS{}", schedule_name, pin_ident);

    let expanded = quote! {
        unsafe {
            static mut _CURRENT: u32 = 0;
            static mut #previous: u32 = 0;
             _CURRENT = millis();

            if  _CURRENT - #previous >= #pin_lit {
                (#closure)();
                #previous =  _CURRENT;
            }
        }

    };

    expanded.into()
}