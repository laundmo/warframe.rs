#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use serde::{
    Deserialize,
    de::DeserializeOwned,
};
use warframe_types::{
    Api,
    Endpoint,
    Language,
};

pub mod worldstate;

pub mod market;

// #[cfg(feature = "profile")]
// pub mod profile;

/// The Error type of this crate
#[derive(Debug, thiserror::Error)]
pub enum Error<T: DeserializeOwned> {
    /// An error from the sent request
    #[error("Couldn't send request: {0}")]
    FaultyRequest(#[from] reqwest::Error),

    /// An error that occurs when the deserialization of `serde_json` fails
    #[error("Couldn't deserialize json body: {0}")]
    FailedDeserialization(#[from] serde_json::Error),

    /// Any error directly from the API
    #[error("Error response from the API: {0}")]
    ApiError(T),
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T, E> {
    Success(T),
    Error(E),
}
pub trait Queryable: Endpoint {
    /// Send a query with the default domain
    fn query(
        client: &reqwest::Client,
        language: Language,
    ) -> impl std::future::Future<
        Output = Result<Self::Return, Error<<Self::Api as Api>::ApiErrorJson>>,
    > + Send {
        Self::query_with_domain(None, client, language)
    }
    /// Send a query, optionally specifying an alternative domain
    fn query_with_domain(
        domain: Option<&str>,
        client: &reqwest::Client,
        language: Language,
    ) -> impl std::future::Future<
        Output = Result<Self::Return, Error<<Self::Api as Api>::ApiErrorJson>>,
    > + Send {
        let mut req = Self::get_parts(language);
        if let Some(domain) = domain {
            req.set_origin(domain);
        }
        async move {
            let url = format!("{}/{}", req.origin, req.path.join("/"));
            let mut builder = client.get(url);
            for q in req.query.iter() {
                builder = builder.query(q);
            }
            for (k, v) in req.header.iter() {
                builder = builder.header(k, v);
            }
            let response = builder
                .send()
                .await?
                .json::<ApiResponse<Self::Return, <Self::Api as Api>::ApiErrorJson>>()
                .await?;
            match response {
                ApiResponse::Success(data) => Ok(data),
                ApiResponse::Error(err) => Err(Error::ApiError(err)),
            }
        }
    }
}
impl<T> Queryable for T where T: Endpoint {}
