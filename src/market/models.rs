use std::fmt::Debug;

use i18n::Language;
use serde::{
    Deserialize,
    de::DeserializeOwned,
};

use super::BASE_URL;
use crate::market::error::Error;
/// Marks a struct as `Queryable`.
///
/// Comes with a default implementation that works universally.
pub trait Queryable: Debug {
    const ENDPOINT: &str;
    type Return: DeserializeOwned + Clone + Debug + Send + Sync + 'static;

    #[must_use]
    fn query(
        client: &reqwest::Client,
        language: Language,
    ) -> impl Future<Output = Result<Self::Return, Error>> + Send {
        async move {
            let response = client
                .get(format!("{}{}", BASE_URL, Self::ENDPOINT))
                .header("Language", language.to_string())
                .send()
                .await?
                .json::<ResponseBase<Self::Return>>()
                .await?;

            if let Some(error) = response.error {
                return Err(Error::Api(error));
            }

            response.data.ok_or(Error::EmptyErrorAndData)
        }
    }
}

macro_rules! impl_queryable {
    ($name:ident,Array, $endpoint:literal) => {
        impl crate::market::Queryable for $name {
            const ENDPOINT: &str = $endpoint;
            type Data = Vec<Self>;
        }
    };
    ($name:ident,Object, $endpoint:literal) => {
        impl crate::market::Queryable for $name {
            const ENDPOINT: &str = $endpoint;
            type Data = Self;
        }
    };
}

impl_queryable!(LichEphemera, Array, "/lich/ephemeras");
impl_queryable!(LichQuirk, Array, "/lich/quirks");
impl_queryable!(LichWeapon, Array, "/lich/weapons");
impl_queryable!(Location, Array, "/locations");
impl_queryable!(Mission, Array, "/missions");
impl_queryable!(Npc, Array, "/npcs");
impl_queryable!(OrderWithUser, Array, "/orders/recent");
impl_queryable!(RivenAttribute, Array, "/riven/attributes");
impl_queryable!(Riven, Array, "/riven/weapons");
impl_queryable!(SisterEphemera, Array, "/sister/ephemeras");
impl_queryable!(SisterQuirk, Array, "/sister/quirks");
impl_queryable!(SisterWeapon, Array, "/sister/weapons");
impl_queryable!(Versions, Object, "/versions");

impl TopOrdersQueryParams {
    pub(crate) fn apply_to(self, mut request_builder: RequestBuilder) -> RequestBuilder {
        append_query!(request_builder, "rank", self.rank);
        append_query!(request_builder, "rank_lt", self.rank_lt);

        append_query!(request_builder, "charges", self.charges);
        append_query!(request_builder, "charges_lt", self.charges_lt);

        append_query!(request_builder, "amber_stars", self.amber_stars);
        append_query!(request_builder, "amber_stars_lt", self.amber_stars_lt);

        append_query!(request_builder, "cyan_stars", self.cyan_stars);
        append_query!(request_builder, "cyan_stars_lt", self.cyan_stars_lt);

        append_query!(request_builder, "subtype", self.subtype);

        request_builder
    }
}

macro_rules! append_query {
    ($builder_ident:ident, $name:literal, $option_value:expr) => {
        if let Some(value) = $option_value {
            $builder_ident = $builder_ident.query(&[($name, value)]);
        }
    };
}

use append_query;
