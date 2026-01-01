use super::*;

//use super::*;
//pub fn expand_kariPH(input: TokenStream) -> TokenStream{}


pub fn expand_kariPH(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);

    if args.len() < 1 || args.len() > 2 {
        return syn::Error::new_spanned(quote! { #args}, 
           "Expected 1 or 2 arguments: signal, [iteration_count]")
           .to_compile_error()
           .into()
    }

    let signal = &args[0];
    let iteration_count = if args.len() == 2 {
        &args[1]
    } else {
        &syn::parse_quote!(10)
    };

    let mut signal_num_ident: Option<u8> = None;
    let mut iteration_count_num_ident: Option<_> = None;

    for (name, expr) in [("signal", signal), ("iteration_count", iteration_count)] {
        if !matches!(expr, Expr::Lit(syn::ExprLit {lit: syn::Lit::Int(_), ..})){
            return syn::Error::new_spanned(expr, format!("{} must be an integer literal", name))
            .to_compile_error()
            .into();
        } else {
            if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(k), ..}) = expr {
                let num = k.base10_parse::<u8>().unwrap();
                if name == "signal" {
                    signal_num_ident = Some(num);
                } else {
                    iteration_count_num_ident = Some(num as usize);
                }
            }
        }
    }


    let signal_num_ident = signal_num_ident.unwrap();
    let iteration_count_num_ident = iteration_count_num_ident.unwrap();

    let expanded = quote! {
        kariPH::new(
            || {
                analogRead!(#signal_num_ident)
            },
            #iteration_count_num_ident
        )
    };

    expanded.into()
}
