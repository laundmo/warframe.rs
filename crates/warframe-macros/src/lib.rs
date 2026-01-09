#[cfg(feature = "worldstate")]
mod derive_timedevent;
mod endpoint;
mod model;

use manyhow::manyhow;
use proc_macro::TokenStream;

/// This is a macro to model a api object.
///
/// Note that this macro will automatically put `#[serde(rename_all = "camelCase")]` on your type.
///
/// Additionally, it will derive `Debug, Clone, PartialEq, PartialOrd, serde::Deserialize` for
/// structs, and `Debug, PartialEq, PartialOrd, Clone, Eq, Copy, Hash, derive_more::Display` for
/// enums.
#[manyhow]
#[proc_macro_attribute]
pub fn model(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    model::expand(args.into(), item.into()).map(Into::into)
}


/// This is a macro which stores data about the API and endpoint with which a type can be requested, 
/// and the form in which it is returned from said API.
/// 
/// It automatically adds [`model`] to the type as well.
/// 
/// Syntax: #[endpoint(ApiMarker:"/path/to/this" -> Self)]
/// The return type `-> Self` can be any type which implements [`serde::DeserializeOwned`]
/// for example `-> Vec<Self>` for APIs returning arrays, or `-> CommonResponse<Self>` for APIs which have some
/// common structure around each endpoints data (for example `{"data": <actual data>, "error": "..."}`)
#[manyhow]
#[proc_macro_attribute]
pub fn endpoint(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    endpoint::expand(args.into(), item.into()).map(Into::into)
}


/// Derives [`warframe_types::worldstate::TimedEvent`]
#[manyhow]
#[proc_macro_derive(TimedEvent)]
pub fn derive_timedevent(item: syn::DeriveInput) -> syn::Result<TokenStream> {
    derive_timedevent::expand(item)
}
