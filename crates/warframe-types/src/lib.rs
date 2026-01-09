use std::error::Error;

use serde::de::DeserializeOwned;
mod language;
pub use language::Language;

pub(crate) mod internal_prelude {
    pub(crate) use warframe_macros::{
        endpoint,
        model,
    };

    #[cfg(feature = "market")]
    pub(crate) use super::market::Market;
    #[cfg(feature = "worldstate")]
    pub(crate) use super::worldstate::Worldstate;
    #[cfg(feature = "worldstate")]
    pub(crate) use crate::worldstate::base::{
        DateTime,
        EventTimes,
        TimedEvent,
    };
}
use internal_prelude::*;

#[cfg(feature = "market")]
pub mod market;
#[cfg(feature = "profile")]
pub mod profile;
#[cfg(feature = "worldstate")]
pub mod worldstate;

#[derive(Default, Debug)]
pub struct HttpParts {
    pub origin: String,
    pub path: Vec<String>,
    pub query: Vec<(String, String)>,
    pub header: Vec<(String, String)>,
}
impl HttpParts {
    pub fn set_origin(&mut self, origin: impl ToString) -> &mut Self {
        self.origin = origin.to_string();
        self
    }
    pub fn set_path(&mut self, path: impl ToString) -> &mut Self {
        let path = path.to_string();
        assert!(path.starts_with("/"));
        self.path = Vec::new();
        self.append_path(path);
        self
    }
    pub fn append_path(&mut self, path: impl ToString) -> &mut Self {
        let path = path.to_string();
        path.split('/')
            .filter(|s| !s.is_empty())
            .for_each(|s| self.path.push(s.to_string()));
        self
    }
    pub fn add_query(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.query.push((
            key.to_string().to_lowercase(),
            value.to_string().to_lowercase(),
        ));
        self
    }
    pub fn add_header(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.header.push((
            key.to_string().to_lowercase(),
            value.to_string().to_lowercase(),
        ));
        self
    }
}

pub trait Api {
    type ErrorJson: DeserializeOwned;
    const DEFAULT_ORIGIN: &str;
    fn new_with_language(language: Language) -> HttpParts {
        let mut parts = HttpParts::default();
        parts.set_origin(Self::DEFAULT_ORIGIN);
        Self::request_apply_language(&mut parts, language);
        parts
    }
    fn request_apply_language(parts: &mut HttpParts, language: Language);
}

pub trait Endpoint {
    type Api: Api;
    const ENDPOINT: &str;
    type Return: DeserializeOwned + Clone + Send + Sync + 'static;

    fn get_parts(language: Language) -> HttpParts {
        let mut parts = Self::Api::new_with_language(language);
        Self::apply_endpoint(&mut parts);
        parts
    }

    fn apply_endpoint(parts: &mut HttpParts) {
        // append, not set, in case the earlier call to Api::request_apply_language
        // set the path to something like /v3/en/ to the path
        parts.append_path(Self::ENDPOINT);
    }
}

#[cfg(test)]
mod test {

    use crate::{
        Api,
        Endpoint,
        HttpParts,
    };

    struct Market;
    impl Api for Market {
        type ErrorJson = ();
        const DEFAULT_ORIGIN: &str = "https://api.warframe.market";

        fn request_apply_language(req: &mut crate::HttpParts, language: super::Language) {
            req.add_header("language", language);
        }
    }
    struct TestEndpoint;
    impl Endpoint for TestEndpoint {
        type Api = Market;
        type Return = ();

        const ENDPOINT: &str = "/items";
        fn apply_endpoint(req: &mut HttpParts) {
            req.append_path(Self::ENDPOINT)
                .add_query("testquery", "testval")
                .add_query("testquery2", "testval2");
        }
    }

    #[test]
    fn test_endpoint() {
        let parts = <TestEndpoint as Endpoint>::get_parts(super::Language::EN);

        assert_eq!(parts.origin, "https://api.warframe.market");
        assert_eq!(parts.path, vec!["items"]);
        assert_eq!(
            parts.query,
            vec![
                ("testquery".to_string(), "testval".to_string()),
                ("testquer2".to_string(), "testval2".to_string())
            ]
        );
        assert_eq!(
            parts.header,
            vec![("language".to_string(), "en".to_string())]
        );
    }
}
