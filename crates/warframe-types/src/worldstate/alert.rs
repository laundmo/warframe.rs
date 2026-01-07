use super::{
    mission::Mission,
    reward_type::RewardType,
};
use crate::internal_prelude::*;

/// An alert in Warframe
#[endpoint(Worldstate:"/alerts" -> Vec<Self>)]

pub struct Alert {
    /// ID of this event
    pub id: String,

    /// The mission associated with the alert
    pub mission: Mission,

    /// The reward type of the alert
    pub reward_types: Vec<RewardType>,
}

#[cfg(test)]
mod test_alert {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Alert;
    use crate::Endpoint;

    type R = <Alert as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/alert.json")]
        #[mode = str]
        alert_en: &str,
    ) {
        from_str::<R>(alert_en).unwrap();
    }
}
