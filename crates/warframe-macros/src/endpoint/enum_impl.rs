use std::collections::VecDeque;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr,
    ExprLit,
    ItemEnum,
    Lit,
    spanned::Spanned,
};

pub fn parse_enum(_args: TokenStream, mut item: ItemEnum) -> syn::Result<TokenStream> {
    // this is here to be able to have the derives at the very top
    let mut vdq = VecDeque::from(item.attrs.clone());
    vdq.push_front(syn::parse_quote!( #[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Copy, Hash, derive_more::Display)] ));

    // check whether the enum has a discriminant, and maybe implement Deserialize_repr
    match item
        .variants
        .first()
        .ok_or_else(|| syn::Error::new(item.span(), "Needs at least 1 variant"))?
        .discriminant
    {
        Some((
            _,
            Expr::Lit(ExprLit {
                lit: Lit::Int(_), ..
            }),
        )) => {
            // repr(u32) for potentially needed flexibility
            vdq.push_front(syn::parse_quote! {
                #[derive(serde_repr::Deserialize_repr)]
            });
            vdq.push_front(syn::parse_quote! {
                #[repr(u32)]
            });
        }
        _ => vdq.push_front(syn::parse_quote!( #[derive(serde::Deserialize)] )),
    };

    item.attrs = vdq.into();

    Ok(quote! {
        #item
    })
}
