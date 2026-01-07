use crate::internal_prelude::*;

/// An Item in Teshin's Shop
#[model]
pub struct SteelPathShopItem {
    /// The i18n Name of the Item
    pub name: String,

    /// The amount of Steel Essence this Item costs
    pub cost: i32,
}

/// Data about steel path offerings
#[endpoint(Worldstate:"/steelPath" -> Self)]
pub struct SteelPath {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// The current weekly offer
    #[serde(rename = "currentReward")]
    pub current_offer: SteelPathShopItem,

    /// The Rotation of Items Teshin offers
    pub rotation: Vec<SteelPathShopItem>,

    /// Items that are always available
    pub evergreens: Vec<SteelPathShopItem>,
}

#[cfg(test)]
mod test_steel_path {
    use rstest::rstest;
    use serde_json::from_str;

    use super::SteelPath;
    use crate::Endpoint;

    type R = <SteelPath as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/steel_path.json")]
        #[mode = str]
        steel_path_en: &str,
    ) {
        from_str::<R>(steel_path_en).unwrap();
    }
}
