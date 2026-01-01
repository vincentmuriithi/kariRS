use syn::ItemStruct;

use super::*;


pub fn expand_eeprom(attr: TokenStream, item: TokenStream) -> TokenStream {
    let addr_lit = parse_macro_input!(attr as LitInt);
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #[derive(Serialize, Deserialize)]
        #input
        impl #name {
            pub fn save(&self, eeprom: &mut kariEEPROM, address: Option<u16>) {
                let mut _address = #addr_lit;
                if let Some(addr) = address{
                    _address = addr;
                }
                let mut buffer = [0u8; size_of::<#name>()];
                let bytes = to_slice(self, &mut buffer).unwrap();
                let _ = eeprom.update(_address, bytes);
            }

            pub fn load(eeprom: &kariEEPROM, address: Option<u16>) -> #name {
                let mut _address = #addr_lit;
                if let Some(addr) = address{
                    _address = addr;
                }
                let mut buffer = [0u8; size_of::<#name>()];
                let _ = eeprom.read(_address, &mut buffer);
                let retrieved: #name = from_bytes(&buffer).unwrap();
                retrieved
            }

            pub fn next_addr(count: u16) -> Option<u16> {
                Some(#addr_lit + count * size_of::<#name>() as u16)
            }
        }
    };
    expanded.into()
}