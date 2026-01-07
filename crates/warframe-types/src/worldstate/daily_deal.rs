use crate::internal_prelude::*;

use super::ItemStringWrapper;

/// Info about the Daily Deal(s)
#[endpoint(Worldstate:"/dailyDeals" -> Vec<Self>)]
pub struct DailyDeal {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// The Item being sold
    pub item: ItemStringWrapper,

    /// The unique name of the Item
    pub unique_name: String,

    /// The original price of the Item
    pub original_price: i32,

    /// The discounted price
    pub sale_price: i32,

    /// The total amount of items available
    pub total: i32,

    /// The number of items sold
    pub sold: i32,

    /// The discount % of the item
    pub discount: i32,
}

#[cfg(test)]
mod test_daily_deal {
    use rstest::rstest;
    use serde_json::from_str;

    use super::DailyDeal;
    use crate::Endpoint;

    type R = <DailyDeal as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/daily_deal.json")]
        #[mode = str]
        daily_deal_en: &str,
    ) {
        from_str::<R>(daily_deal_en).unwrap();
    }
}
