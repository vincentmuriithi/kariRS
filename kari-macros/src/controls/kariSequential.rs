use super::*;



pub fn expand_kariSequential(input: TokenStream) -> TokenStream {
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

    let mut pins = Vec::new();
    for expr in pins_array.elems {
        if let Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(lit), ..}) = expr {
            pins.push(lit.clone());
        } else {
            return syn::Error::new_spanned(expr, "Pins must be integer literals")
            .to_compile_error()
            .into();
        }
    }

    let mut arms = Vec::new();

    for (index, lit) in pins.iter().enumerate() {
        let prev = if index == 0 { &pins[pins.len() - 1]} else { &pins[index - 1]};
        let current = lit;
        arms.push(
            quote! {
                #index => { digitalWrite!(#prev, LOW); digitalWrite!(#current, HIGH);}
            }
        );
    }


    let static_name = format_ident!("{}_SEQUENCE_STATE", name_ident.to_string().to_uppercase());

    let expanded = quote! {
        unsafe {
            static mut #static_name: usize = 0;
            let pins = [#(#pins),*];

            match #static_name {
                #(#arms)*
                _ => {}
            }

            #static_name = (#static_name + 1) % pins.len();

        }
    };
    expanded.into()
}