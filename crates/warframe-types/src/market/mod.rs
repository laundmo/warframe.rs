pub mod activity;
mod error;
pub mod item;
pub mod item_short;
pub mod lich_ephemera;
pub mod lich_quirk;
pub mod lich_weapon;
pub mod location;
pub mod mission;
pub mod npc;
pub mod order;
pub mod order_with_user;
pub mod riven;
pub mod riven_attribute;
pub mod riven_group;
pub mod riven_type;
pub mod set_items;
pub mod sister_ephemera;
pub mod sister_quirk;
pub mod sister_weapon;
pub mod top_orders;
pub mod top_orders_query_params;
pub mod user_short;
pub mod versions;
use std::collections::HashMap;

pub use error::Error;
use serde::Deserialize;

use crate::{
    Api,
    Language,
};

pub type I18N<T> = HashMap<Language, T>;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBase<T> {
    pub api_version: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub struct Market;
impl Api for Market {
    const DEFAULT_ORIGIN: &str = "https://api.warframe.market";
    type ApiErrorJson = Option<String>;

    fn request_apply_language(parts: &mut crate::HttpParts, language: Language) {
        parts.add_header("language", language.get_market_string());
    }
}
