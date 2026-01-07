use manyhow::bail;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data,
    Fields,
    Index,
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

#[cfg(feature = "worldstate")]
pub fn expand(item: syn::DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = item.ident;
    const TIMED_NAME: &str = "Timed";
    let Data::Struct(ref data) = item.data else {
        bail!("cannot derive TimedEvent for non-struct types.");
    };
    let timed = first_member_of_type(&data.fields, TIMED_NAME);

    let timed_event_trait_impl = timed.map(|timef| {
        quote! {
            impl crate::worldstate::TimedEvent for #struct_name {
                #[doc = "Returns the time when this event began"]
                fn activation(&self) -> chrono::DateTime<chrono::Utc> {
                    self.#timef.activation
                }
                #[doc = "Returns the time when this event ends"]
                fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
                    self.#timef.expiry
                }
            }
        }
    });

    Ok(quote! {#timed_event_trait_impl}.into())
}

#[cfg(not(feature = "worldstate"))]
pub fn expand(item: syn::DeriveInput) -> syn::Result<TokenStream> {
    Ok(quote! {}.into())
}
