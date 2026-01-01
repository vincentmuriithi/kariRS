use super::*;



pub fn expand_kprintln(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    let mut elems = expr.iter();
    let formatted_string = if let Some(fmt) = elems.next() {
        fmt
    } else {
        return syn::Error::new_spanned(expr, "kprintln! requires at least a format string")
        .to_compile_error()
        .into();
    };

    let variables: Vec<_> = elems.collect();

    quote! {
        ufmt::uwriteln!(&mut _kari_serial, #formatted_string #(,#variables)*).unwrap();
    }.into()
}