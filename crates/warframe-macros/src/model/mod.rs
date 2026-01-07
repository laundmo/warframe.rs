use std::collections::VecDeque;

use enum_impl::parse_enum;
use proc_macro2::{
    Span,
    TokenStream,
};
use quote::ToTokens;
use syn::{
    Item,
    ItemStruct,
    spanned::Spanned,
};

mod enum_impl;

pub fn parse_struct(args: TokenStream, mut item: ItemStruct) -> syn::Result<TokenStream> {
    let mut vdq = VecDeque::from(item.attrs.clone());
    vdq.push_front(syn::parse_quote!(#[serde(rename_all = "camelCase")]));
    vdq.push_front(syn::parse_quote!(#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::PartialEq, serde::Deserialize)]));
    item.attrs = vdq.into();

    Ok(item.to_token_stream())
}

pub fn expand(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    match syn::parse2::<Item>(item)? {
        Item::Enum(enum_item) => parse_enum(args, enum_item),
        Item::Struct(struct_item) => parse_struct(args, struct_item),
        item => Err(syn::Error::new(
            item.span(),
            "Only structs and enums are supported",
        )),
    }
}
