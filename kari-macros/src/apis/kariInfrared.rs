use super::*;



pub fn expand_kariInfrared(input: TokenStream) -> TokenStream {
    let arg = parse_macro_input!(input as syn::ExprLit);
    let expr = syn::Expr::Lit(arg);
    let signal_literal;

    match expect_int(&expr, "Infrared pin"){
        Ok(num) => 
        signal_literal = syn::LitInt::new(num.to_string().as_str(), proc_macro2::Span::call_site()),
        Err(err) => return err
    }

    let expanded = quote! {
        kariInfrared::new(
            || digitalRead!(#signal_literal) != 0
        )
    };

    expanded.into()
}