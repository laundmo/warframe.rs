mod args;

use std::collections::VecDeque;

use args::Args;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Fields,
    Index,
    ItemStruct,
    Member,
    Type,
};

fn matches_type_name(ty: &Type, name: &str) -> bool {
    if let Type::Path(path) = ty {
        path.path.get_ident().is_some_and(|i| i == name)
    } else {
        false
    }
}

fn first_member_of_type(fields: &Fields, type_name: &str) -> Option<Member> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .find(|f| matches_type_name(&f.ty, type_name))
            .map(|f| Member::Named(f.ident.as_ref().unwrap().clone())),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .find(|(_, f)| matches_type_name(&f.ty, type_name))
            .map(|(i, _)| Member::Unnamed(Index::from(i))),
        _ => None,
    }
}

pub fn parse_struct(args: TokenStream, mut item: ItemStruct) -> syn::Result<TokenStream> {
    let mut vdq = VecDeque::from(item.attrs.clone());
    vdq.push_front(syn::parse_quote!(#[serde(rename_all = "camelCase")]));
    vdq.push_front(syn::parse_quote!(#[derive(Debug, Clone, PartialEq, serde::Deserialize)]));
    item.attrs = vdq.into();

    // panic!("{:?}", item.attrs);
    const TIMED_NAME: &str = "Timed";

    let timed = first_member_of_type(&item.fields, TIMED_NAME);

    let struct_name = &item.ident;

    let args: Args = syn::parse2(args)?;

    let timed_event_trait_impl = timed.map(|timef| {
        quote! {
            impl crate::TimedGetter for #struct_name {
                #[doc = "Returns the Timed for this event"]
                fn get(&self) -> Timed {
                    self.#timef
                }
            }
        }
    });

    let endpoint_value = args.endpoint.value();
    let return_type = args.return_type;
    let api_type = args.api;

    Ok(quote! {
        #item

        #timed_event_trait_impl

        impl crate::Endpoint for #struct_name {
            type Return = #return_type;
            type Api = #api_type;
            const ENDPOINT: &str = #endpoint_value;
        }

    })
}
