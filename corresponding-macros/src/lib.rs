use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_macro_input, parse_quote, AngleBracketedGenericArguments, GenericArgument, Ident, Item,
    ItemMod, Path, PathArguments, Stmt, Type, TypePath,
};

#[derive(Debug)]
struct OptionType {
    pub ident: Ident,
    pub option: bool,
}

#[proc_macro_attribute]
pub fn derive_corresponding(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);

    if let Some((_, ref mut items)) = input.content {
        let structs = get_structs(&items);

        for l in &structs {
            for r in &structs {
                if l != r {
                    items.push(generate_move_corresponding_impl(l, r));
                }
            }
        }
    }

    TokenStream::from(input.into_token_stream())
}

fn get_structs(items: &Vec<Item>) -> Vec<syn::ItemStruct> {
    items
        .iter()
        .cloned()
        .filter_map(|item| match item {
            Item::Struct(item_struct) => Some(item_struct.clone()),
            _ => None,
        })
        .collect()
}

fn generate_move_corresponding_impl(l: &syn::ItemStruct, r: &syn::ItemStruct) -> Item {
    let mut statements: Vec<Stmt> = vec![];

    for l_field in &l.fields {
        for r_field in &r.fields {
            let l_field_ident = &l_field.ident;
            let r_field_ident = &r_field.ident;

            if let Some(l_type) = get_type(&l_field.ty) {
                if let Some(r_type) = get_type(&r_field.ty) {
                    if l_field_ident == r_field_ident && l_type.ident == r_type.ident {
                        match (l_type.option, r_type.option) {
                            (false, false) => statements.push(parse_quote! { self. #l_field_ident = rhs. #r_field_ident ; }),
                            (true, false) => statements.push(parse_quote! { self. #l_field_ident = Some ( rhs. #r_field_ident ) ; }),
                            (false, true) => statements.push(parse_quote! { if let Some ( r ) = rhs. #r_field_ident { self. #l_field_ident = r } ; }),
                            (true, true) => statements.push(parse_quote! { if rhs. #r_field_ident .is_some() { self. #l_field_ident = rhs. #r_field_ident } ; }),
                        }
                    }
                }
            }
        }
    }

    let l_id = &l.ident;
    let r_id = &r.ident;

    parse_quote! {
        impl ::corresponding::MoveCorresponding< #r_id > for #l_id {
            #[inline]
            fn move_corresponding(&mut self, rhs: #r_id ) {
                #(#statements)*
            }
        }
    }
}

fn get_type(ty: &syn::Type) -> Option<OptionType> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(segment) = segments.first() {
            if segment.ident.to_string().as_str() == "Option" {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    args, ..
                }) = segment.arguments.clone()
                {
                    if let Some(GenericArgument::Type(Type::Path(TypePath {
                        path: Path { segments, .. },
                        ..
                    }))) = args.first()
                    {
                        if let Some(segment) = segments.first() {
                            return Some(OptionType {
                                ident: segment.ident.clone(),
                                option: true,
                            });
                        }
                    }
                }
            } else {
                return Some(OptionType {
                    ident: segment.ident.clone(),
                    option: false,
                });
            }
        }
    }
    None
}
