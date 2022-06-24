use proc_macro::TokenStream;
use quote::{ToTokens, __private::TokenTree};
use syn::{
    parse_macro_input, parse_quote, AngleBracketedGenericArguments, GenericArgument, Ident, Item,
    ItemMod, Path, PathArguments, PathSegment, Stmt, Type, TypePath,
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
                    if has_default_derive(l) {
                        items.push(generate_from_impl(l, r));
                    }
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
    let l_ident = &l.ident;
    let r_ident = &r.ident;
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

    parse_quote! {
        impl ::corresponding::MoveCorresponding< #r_ident > for #l_ident {
            #[inline]
            fn move_corresponding(&mut self, rhs: #r_ident ) {
                #(#statements)*
            }
        }
    }
}

fn has_default_derive(l: &syn::ItemStruct) -> bool {
    for attribute in l.clone().attrs {
        if let Some(PathSegment { ident, .. }) = attribute.path.segments.first() {
            if ident.to_string().as_str() == "derive" {
                for token_tree in attribute.tokens {
                    if let TokenTree::Group(group) = token_tree {
                        for token_tree2 in group.stream() {
                            if let TokenTree::Ident(ident) = token_tree2 {
                                if ident.to_string().as_str() == "Default" {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn generate_from_impl(l: &syn::ItemStruct, r: &syn::ItemStruct) -> Item {
    let l_ident = &l.ident;
    let r_ident = &r.ident;

    parse_quote! {
        impl ::std::convert::From< #r_ident > for #l_ident {
            #[inline]
            fn from(rhs: #r_ident ) -> Self {
                use ::corresponding::MoveCorresponding;
                let mut lhs = #l_ident ::default();
                lhs.move_corresponding(rhs);
                lhs
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
