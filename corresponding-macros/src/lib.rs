use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, Item, ItemMod, Stmt};

#[proc_macro_attribute]
pub fn derive_corresponding(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);
    if let Some((_, ref mut items)) = input.content {
        let structs: Vec<_> = items
            .iter()
            .filter_map(|item| {
                if let Item::Struct(item_struct) = item {
                    Some(item_struct.clone())
                } else {
                    None
                }
            })
            .collect();

        for l in &structs {
            for r in &structs {
                if l != r {
                    let mut statements: Vec<Stmt> = vec![];
                    for l_field in &l.fields {
                        for r_field in &r.fields {
                            let l_field_id = &l_field.ident;
                            let r_field_id = &r_field.ident;
                            let l_field_id_string = l_field_id.clone().unwrap().to_string();
                            let r_field_id_string = r_field_id.clone().unwrap().to_string();

                            if l_field_id_string == r_field_id_string && l_field.ty == r_field.ty {
                                statements
                                    .push(parse_quote! { self. #l_field_id = rhs. #r_field_id ; });
                            }
                        }
                    }

                    let l_id = &l.ident;
                    let r_id = &r.ident;
                    items.push(parse_quote! {
                        impl ::corresponding::MoveCorresponding< #r_id > for #l_id {
                            #[inline]
                            fn move_corresponding(&mut self, rhs: #r_id ) {
                                #(#statements)*
                            }
                        }
                    });
                }
            }
        }
    }

    TokenStream::from(input.into_token_stream())
}
