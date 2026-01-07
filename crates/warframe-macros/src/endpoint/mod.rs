use std::collections::VecDeque;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Item,
    ItemStruct,
    LitStr,
    Token,
    TypePath,
    parse::{
        Parse,
        ParseStream,
    },
    spanned::Spanned,
};

pub fn expand(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    match syn::parse2::<Item>(item)? {
        Item::Struct(struct_item) => parse_struct(args, struct_item),
        item => Err(syn::Error::new(
            item.span(),
            "Only structs and enums are supported",
        )),
    }
}

pub struct Args {
    pub api: TypePath,
    pub endpoint: LitStr,
    pub return_type: TypePath,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let api: TypePath = input.parse()?;
        let _: Token![:] = input.parse()?;
        let endpoint: LitStr = input.parse()?;
        let _: Token![->] = input.parse()?;
        let return_type: TypePath = input.parse()?;

        if !endpoint.value().starts_with('/') {
            return Err(syn::Error::new_spanned(
                endpoint,
                "endpoint path must start with a `/`",
            ));
        }

        Ok(Self {
            api,
            endpoint,
            return_type,
        })
    }
}

pub fn parse_struct(args: TokenStream, mut item: ItemStruct) -> syn::Result<TokenStream> {
    let mut vdq = VecDeque::from(item.attrs.clone());
    vdq.push_front(syn::parse_quote!(#[::warframe_macros::model]));
    item.attrs = vdq.into();
    let args: Args = syn::parse2(args)?;
    let struct_name = &item.ident;
    // panic!("{:?}", item.attrs);

    let endpoint_value = args.endpoint.value();
    let return_type = args.return_type;
    let api_type = args.api;

    Ok(quote! {
        #item

        impl crate::Endpoint for #struct_name {
            type Return = #return_type;
            type Api = #api_type;
            const ENDPOINT: &str = #endpoint_value;
        }

    })
}
