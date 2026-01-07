use crate::internal_prelude::*;

/// Any current modifiers applied to all users, such as double drops, double XP, etc.
#[endpoint(Worldstate:"/globalUpgrades" -> Vec<Self>)]
pub struct GlobalUpgrade {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// What kind of upgrade
    pub upgrade: String,

    /// Operation descriptor
    pub operation: String,

    /// Symbol corresponding to operation
    pub operation_symbol: String,

    /// Value corresponding to performing the operation
    pub upgrade_operation_value: i32,

    /// Whether the upgrade has expired
    pub expired: bool,
}

#[cfg(test)]
mod test_global_upgrade {
    use rstest::rstest;
    use serde_json::from_str;

    use super::GlobalUpgrade;
    use crate::Endpoint;

    type R = <GlobalUpgrade as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/global_upgrade.json")]
        #[mode = str]
        global_upgrade_en: &str,
    ) {
        from_str::<R>(global_upgrade_en).unwrap();
    }
}
