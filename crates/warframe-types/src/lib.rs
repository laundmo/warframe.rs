use derive_more::Display;
use serde::de::DeserializeOwned;

#[cfg(feature = "market")]
pub mod market;
#[cfg(feature = "profile")]
pub mod profile;
#[cfg(feature = "worldstate")]
pub mod worldstate;

#[derive(Default, Debug)]
pub struct BasicRequest {
    schema: String,
    host: String,
    path: Vec<String>,
    query: Vec<(String, String)>,
    header: Vec<(String, String)>,
}
impl BasicRequest {
    pub fn get_url(&self) -> String {
        let path = self.path.join("/");
        let mut base_url = format!("{}://{}/{}", self.schema, self.host, path);
        for (i, (k, v)) in self.query.iter().enumerate() {
            if i == 0 {
                base_url += &format!("?{k}={v}");
            } else {
                base_url += &format!("&{k}={v}");
            }
        }
        base_url
    }
    pub fn set_schema(&mut self, schema: impl ToString) -> &mut Self {
        self.schema = schema.to_string();
        self
    }
    pub fn set_host(&mut self, host: impl ToString) -> &mut Self {
        self.host = host.to_string();
        self
    }
    pub fn set_domain(&mut self, domain: impl ToString) -> &mut Self {
        let domain = domain.to_string();
        let (schema, host) = domain
            .split_once("://")
            .expect("Could not split domain into schema and host parts");
        self.set_schema(schema).set_host(host)
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
    pub fn path_mut(&mut self) -> &mut [String] {
        &mut self.path
    }
    pub fn add_query(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.query.push((
            key.to_string().to_lowercase(),
            value.to_string().to_lowercase(),
        ));
        self
    }
    pub fn query_mut(&mut self) -> &mut [(String, String)] {
        &mut self.query
    }
    pub fn add_header(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.header.push((
            key.to_string().to_lowercase(),
            value.to_string().to_lowercase(),
        ));
        self
    }
    pub fn header_mut(&mut self) -> &mut [(String, String)] {
        &mut self.header
    }
}

// TODO: Temp while testing
#[derive(Display)]
pub enum Language {
    en,
}

trait Api {
    const DEFAULT_DOMAIN: &str;
    fn request() -> BasicRequest {
        let mut r = BasicRequest::default();
        r.set_domain(Self::DEFAULT_DOMAIN);
        r
    }
    fn request_apply_language(req: &mut BasicRequest, language: Language);
}

trait Endpoint {
    type Api: Api;
    const ENDPOINT: &str;
    type Return: DeserializeOwned;

    fn apply_endpoint(req: &mut BasicRequest) {
        req.append_path(Self::ENDPOINT);
    }
}

// trait Queryable: Endpoint {
//     /// Send a query with the default domain
//     fn query(client: dummy::Dummy, language: dummy::Language) -> String {
//         Self::query_with_domain(None, client, language)
//     }
//     /// Send a query, optionally specifying the domain
//     fn query_with_domain(
//         domain: Option<&str>,
//         client: dummy::Dummy,
//         language: dummy::Language,
//     ) -> String {
//         let mut req = Self::Api::request();
//         Self::Api::request_apply_language(&mut req, language);
//         Self::apply_endpoint(&mut req);

//         // client.request(req)
//     }
// }
// impl<T> Queryable for T where T: Endpoint {}

#[cfg(test)]
mod test {

    use crate::{
        Api,
        BasicRequest,
        Endpoint,
    };
    pub struct Dummy;
    impl Dummy {
        pub fn request(&self, req: BasicRequest) -> (String, Vec<(String, String)>) {
            (req.get_url(), req.header)
        }
    }

    struct Market;
    impl Api for Market {
        const DEFAULT_DOMAIN: &str = "https://api.warframe.market";

        fn request_apply_language(req: &mut crate::BasicRequest, language: super::Language) {
            req.add_header("language", language);
        }
    }
    struct TestEndpoint;
    impl Endpoint for TestEndpoint {
        type Api = Market;
        type Return = ();

        const ENDPOINT: &str = "/items";
        fn apply_endpoint(req: &mut BasicRequest) {
            req.append_path(Self::ENDPOINT)
                .add_query("testquery", "testval")
                .add_query("testquery2", "testval2");
        }
    }

    #[test]
    fn test_endpoint() {
        let mut req = <TestEndpoint as Endpoint>::Api::request();
        <TestEndpoint as Endpoint>::Api::request_apply_language(&mut req, super::Language::en);
        <TestEndpoint as Endpoint>::apply_endpoint(&mut req);

        assert_eq!(
            (
                "https://api.warframe.market/items?testquery=testval&testquery2=testval2"
                    .to_owned(),
                vec![("language".to_owned(), "en".to_owned())]
            ),
            Dummy.request(req)
        );
    }
}
