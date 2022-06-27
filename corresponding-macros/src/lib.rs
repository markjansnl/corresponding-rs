//! [![github]](https://github.com/markjansnl/corresponding-rs)&ensp;[![crates-io]](https://crates.io/crates/corresponding-macros)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This crate provides the procedural macros used in the [corresponding] crate.
//!
//! [corresponding]: https://docs.rs/corresponding/

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

/// Use this macro on a module to generate [MoveCorresponding] implementations for all
/// structs in this module. For all structs deriving [Default] also the [From] trait will
/// be implemented.
///
/// # Example
///
/// ```no_run
/// use corresponding::derive_corresponding;
///
/// #[derive_corresponding]
/// mod my_mod {
///     #[derive(Debug, Default)]
///     pub struct A {
///         pub a: u8,
///         pub b: u8,
///         pub c: u8,
///     }
///
///     #[derive(Debug, Clone)]
///     pub struct B {
///         pub a: u8,
///         pub b: Option<u8>,
///         pub d: u8,
///     }
/// }
/// ```
///
/// You can also put the attribute on an external module. This is not supported in rustdoc, but is supported in the real world.
///
/// ```no_run,compile_fail
/// #[derive_corresponding]
/// mod my_other_mod;
/// ```
///
/// This will implement [MoveCorresponding] for all combinations of structs within the crate
/// module. The generated implementations are zero cost abstractions and will look like:
///
/// ```no_run
/// # mod my_mod {
/// #     #[derive(Default)]
/// #     pub struct A {
/// #         pub a: u8,
/// #         pub b: u8,
/// #         pub c: u8,
/// #     }
/// #
/// #     pub struct B {
/// #         pub a: u8,
/// #         pub b: Option<u8>,
/// #         pub d: u8,
/// #     }
/// # }
/// # use corresponding::MoveCorresponding;
/// # use my_mod::*;
/// impl MoveCorresponding<B> for A {
///     fn move_corresponding(&mut self, rhs: B) {
///         self.a = rhs.a;
///         if let Some(r) = rhs.b {
///             self.b = r;
///         }
///     }
/// }
///
/// impl MoveCorresponding<A> for B {
///     fn move_corresponding(&mut self, rhs: A) {
///         self.a = rhs.a;
///         self.b = Some(rhs.b);
///     }
/// }
/// ```
///
/// Because struct A derives [Default], it will also implement [From]. The generated
/// implementation looks like:
///
/// ```no_run
/// # mod my_mod {
/// #     #[derive(Default)]
/// #     pub struct A {
/// #         pub a: u8,
/// #         pub b: u8,
/// #         pub c: u8,
/// #     }
/// #
/// #     pub struct B {
/// #         pub a: u8,
/// #         pub b: Option<u8>,
/// #         pub d: u8,
/// #     }
/// # }
/// # use corresponding::MoveCorresponding;
/// # impl MoveCorresponding<B> for A {
/// #     fn move_corresponding(&mut self, rhs: B) {
/// #         self.a = rhs.a;
/// #         if let Some(r) = rhs.b {
/// #             self.b = r;
/// #         }
/// #     }
/// # }
/// # use my_mod::*;
/// impl From<B> for A {
///     fn from(rhs: B) -> Self {
///         let mut lhs = A::default();
///         lhs.move_corresponding(rhs);
///         lhs
///     }
/// }
/// ```
///
/// [MoveCorresponding]: https://docs.rs/corresponding/trait.MoveCorresponding.html
///
#[proc_macro_attribute]
pub fn derive_corresponding(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);

    if let Some((_, ref mut items)) = input.content {
        let structs = get_structs(items);

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

/// Get the structs at top level of the module
fn get_structs(items: &[Item]) -> Vec<syn::ItemStruct> {
    items
        .iter()
        .cloned()
        .filter_map(|item| match item {
            Item::Struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .collect()
}

/// Generate the `impl MoveCorresponding<Right> for Left` from two ItemStructs
fn generate_move_corresponding_impl(l: &syn::ItemStruct, r: &syn::ItemStruct) -> Item {
    let l_ident = &l.ident;
    let r_ident = &r.ident;

    // Prepare the statements
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

    // Generate the impl
    parse_quote! {
        impl ::corresponding::MoveCorresponding< #r_ident > for #l_ident {
            #[inline]
            fn move_corresponding(&mut self, rhs: #r_ident ) {
                #(#statements)*
            }
        }
    }
}

/// Check whether the given struct has `#[derive(Default)]` attribute
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

/// Generate `impl From<Right> for Left`
/// Just construct a new object by using the Default trait
/// and copy the corresponding fields
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

/// Get the type of a field
/// When the type is Option<T>: return type T and option: true
/// Else, for type T: return type T and option: false
/// If we don't know the type, return a None, so the field will not be copied over
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
