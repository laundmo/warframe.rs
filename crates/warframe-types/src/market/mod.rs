pub mod activity;
pub mod i18n;
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

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBase<T> {
    pub api_version: String,
    pub data: Option<T>,
    pub error: Option<String>,
}
