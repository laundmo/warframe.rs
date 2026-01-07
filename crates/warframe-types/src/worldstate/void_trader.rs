use crate::internal_prelude::*;

use super::ItemStringWrapper;

/// An Item in Baro's inventory
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct VoidTraderInventoryItem {
    /// The item that is being sold
    pub item: ItemStringWrapper,

    /// The Ducat cost of this item
    pub ducats: i32,

    /// The Credit cost of this item
    pub credits: i32,
}

/// Information on the current Void Trader offerings, or when he will arrive
#[endpoint(Worldstate:"/voidTrader" -> Self)]
pub struct VoidTrader {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The i18n of the Node Baro will appear on
    pub location: String,

    /// Baro's Inventory
    pub inventory: Vec<VoidTraderInventoryItem>,
}

#[cfg(test)]
mod test_void_trader {
    use rstest::rstest;
    use serde_json::from_str;

    use super::VoidTrader;
    use crate::Endpoint;

    type R = <VoidTrader as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/void_trader.json")]
        #[mode = str]
        void_trader_en: &str,
    ) {
        from_str::<R>(void_trader_en).unwrap();
    }
}
