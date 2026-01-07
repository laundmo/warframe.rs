use crate::internal_prelude::*;

/// The State of the Cambion Drift
#[model]
#[serde(rename_all = "lowercase")]
pub enum CambionDriftState {
    /// The 'Vome' state
    Vome,
    /// The 'Fass' state
    Fass,
}

/// Cambion Drift info
#[endpoint(Worldstate:"/cambionCycle" -> Self)]
pub struct CambionDrift {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// The id of the cycle
    pub id: String,

    /// The state of the cambion drift (vome/fass)
    pub state: CambionDriftState,
}

#[cfg(test)]
mod test_cambion_drift {
    use rstest::rstest;
    use serde_json::from_str;

    use super::CambionDrift;
    use crate::Endpoint;

    type R = <CambionDrift as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/cambion_drift.json")]
        #[mode = str]
        cambion_drift_en: &str,
    ) {
        from_str::<R>(cambion_drift_en).unwrap();
    }
}
