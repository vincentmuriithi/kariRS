use super::*;



pub fn expand_kariBegin(input: TokenStream) -> TokenStream {
     let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    
    let pin_mode = &args[1];

    let pins_array = if let Expr::Array(arr) = &args[0] {
        arr.clone()
    } else{
        return syn::Error::new_spanned(&args[1],"First argument must be an array").to_compile_error().into();
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

    if let Expr::Path(path ) = pin_mode {
        let ident = &path.path.segments.last().unwrap().ident;

        if ident.to_string() != "OUTPUT" && ident.to_string() != "INPUT" && 
            ident.to_string() != "PWM" && ident.to_string() != "_INPUT" {
            return syn::Error::new_spanned(pin_mode, "pin mode must be OUTPUT or INPUT")
            .to_compile_error()
            .into();
        }
    } else {
         return syn::Error::new_spanned(pin_mode, "pin mode must be OUTPUT or INPUT")
            .to_compile_error()
            .into();
    }
    let expanded = quote! {
        #(
            pinMode!(#pins, #pin_mode);
        )*
    };

    expanded.into()
}