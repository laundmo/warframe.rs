use serde::Deserialize;

use super::{
    
    order::Order,
    user_short::UserShort,
};



/// Represents the `/orders/recent` endpoint.
/// Get the most recent orders.
/// 500 max, for the last 4 hours, sorted by the [`OrderWithUser::order`]'s [`Order::created_at`]
/// field.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderWithUser {
    /// The order details.
    #[serde(flatten)]
    pub order: Order,

    /// Represents the user who created the order, with basic profile information.
    pub user: UserShort,
}

#[cfg(test)]
mod test {
    use super::OrderWithUser;
    use crate::market::{
        
        ResponseBase,
    };

    #[rstest::rstest]
    fn order_with_user(
        #[files("src/market/fixtures/orders.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<OrderWithUser>>(json)?;

        Ok(())
    }
}
