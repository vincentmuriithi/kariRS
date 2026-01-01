use super::*;

#[allow(non_snake_case)]
pub fn expand_kariDrive(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated<Expr, Token![,]>::parse_terminated);
    
    if let Err(err) = check_num_of_pins(&args, 6) {
        return err
    }

    for k in 0..args.len() {
        if let Err(err) = expect_int(&args[k], "Pin") {
            return err
        }
    }

    let motor1_en = match expect_int(&args[0], "motor1_en"){
        Ok(value) => syn::LitInt::new(&format!("{}u8", value), proc_macro2::Span::call_site()),
        Err(err) => return err
    };  
    let motor1_in_a = match expect_int(&args[1], " motor1_in_a"){
        Ok(value) => syn::LitInt::new(&format!("{}u8", value), proc_macro2::Span::call_site()),
        Err(err) => return err
    };  
    let motor1_in_b = match expect_int(&args[2], "motor1_in_b"){
        Ok(value) => value,
        Err(err) => return err
    };   
    let motor2_en = match expect_int(&args[3], "motor2_en"){
        Ok(value) => syn::LitInt::new(&format!("{}u8", value), proc_macro2::Span::call_site()),
        Err(err) => return err
    };    
    let motor2_in_a = match expect_int(&args[4], "motor2_in_a"){
        Ok(value) => syn::LitInt::new(&format!("{}u8", value), proc_macro2::Span::call_site()),
        Err(err) => return err
    };  
    let motor2_in_b = match expect_int(&args[5], "motor2_in_b"){
        Ok(value) => syn::LitInt::new(&format!("{}u8", value), proc_macro2::Span::call_site()),
        Err(err) => return err
    };

    let motor1_en_arms = generate_analog_write_arms(&motor1_en); 
    let motor2_en_arms = generate_analog_write_arms(&motor2_en);
    

    let expanded = quote! {
        kariDrive::new(
            |speed| {
                match speed {
                    #(#motor1_en_arms)*
                    _ => {}
                }
            },
            |value| {
                if value {
                    digitalWrite!(#motor1_in_a, HIGH);
                } else {
                    digitalWrite!(#motor1_in_a, LOW);
                }
            },
            |value| {
                if value {
                    digitalWrite!(#motor1_in_b, HIGH);
                } else {
                    digitalWrite!(#motor1_in_b, LOW);
                }
            },


            |speed: u8| {
                match speed {
                    #(#motor2_en_arms)*
                    _ => {}
                }
            },
            |value| {
                if value {
                    digitalWrite!(#motor2_in_a, HIGH);
                } else {
                    digitalWrite!(#motor2_in_a, LOW);
                }
            },
            |value| {
                if value {
                    digitalWrite!(#motor2_in_b, HIGH);
                } else {
                    digitalWrite!(#motor2_in_b, LOW);
                }
            },

        )

       
    };
    expanded.into()
}