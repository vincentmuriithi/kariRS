use super::*;



pub fn expand_kariPulse(input: TokenStream) -> TokenStream {
     let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    
    let pins_array = if let Expr::Array(arr) = &args[0] {
        arr.clone()
    } else{
        return syn::Error::new_spanned(&args[1],"First argument must be an array").to_compile_error().into();
    };

    let name_ident = if let Expr::Path(path) = &args[1] {
        path.path.get_ident().unwrap().clone()
    } else{
        return syn::Error::new_spanned(&args[1],"Second argument must be an identifier").to_compile_error().into();
    };

    let mut calls_high = Vec::new();
    let mut calls_low = Vec::new();

    for expr in pins_array.elems{
        if let Expr::Lit(syn::ExprLit {lit: syn::Lit::Int(lit), ..}) = expr {
            calls_high.push(quote! { digitalWrite!(#lit, HIGH);});
            calls_low.push(quote! { digitalWrite!(#lit, LOW);});
        } else {
            return syn::Error::new_spanned(expr, "Pins must be integer literals")
            .to_compile_error()
            .into();
        }
    }

    let static_name = format_ident!("{}_STATE", name_ident.to_string().to_uppercase());

    let expanded = quote! {
        unsafe {
            static mut #static_name: bool = false;

            if #static_name {
                #(#calls_high)*
            } else {
                #(#calls_low)*
            }
            #static_name = !#static_name;
        }

    };

    expanded.into()
}