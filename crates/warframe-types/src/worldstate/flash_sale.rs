use crate::internal_prelude::*;

use super::ItemStringWrapper;

/// Popular Deals, discounts, featured deals
#[endpoint(Worldstate:"/flashSales" -> Vec<Self>)]
pub struct FlashSale {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// The item being sold
    pub item: ItemStringWrapper,

    /// The discount of the Item
    pub discount: i32,

    /// The PLATINUM price of this item
    pub premium_override: i32,

    /// The CREDIT price of this item
    pub regular_override: i32,

    /// Whether the item is popular or not
    pub is_popular: Option<bool>,

    /// Whether the item is featured or not
    pub is_featured: Option<bool>,
}

#[cfg(test)]
mod test_flash_sale {
    use rstest::rstest;
    use serde_json::from_str;

    use super::FlashSale;
    use crate::Endpoint;

    type R = <FlashSale as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/flash_sale.json")]
        #[mode = str]
        flash_sale_en: &str,
    ) {
        from_str::<R>(flash_sale_en).unwrap();
    }
}
