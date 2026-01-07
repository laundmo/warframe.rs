use crate::internal_prelude::*;

/// Represents the state on Orb Vallis
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrbVallisState {
    /// Warm
    Warm,
    /// Cold
    Cold,
}

/// The current cycle of the Orb Vallis warm/cold cycle
#[endpoint(Worldstate:"/vallisCycle" -> Self)]
pub struct OrbVallis {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The state of the orb vallis (warm/cold)
    pub state: OrbVallisState,
}

#[cfg(test)]
mod test_orb_vallis {
    use rstest::rstest;
    use serde_json::from_str;

    use super::OrbVallis;
    use crate::Endpoint;

    type R = <OrbVallis as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/orb_vallis.json")]
        #[mode = str]
        orb_vallis_en: &str,
    ) {
        from_str::<R>(orb_vallis_en).unwrap();
    }
}
