use crate::internal_prelude::*;

/// Represents the current state on cetus
#[model]
#[serde(rename_all = "lowercase")]
pub enum CetusState {
    /// Represents Cetus' day state
    Day,
    /// Represents Cetus' night state
    Night,
}

/// The Information about cetus
#[derive(
    ::warframe_macros::TimedEvent,
    ::core::fmt::Debug,
    ::core::clone::Clone,
    ::core::cmp::PartialEq,
    serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[doc = " The Information about cetus"]
pub struct Cetus {
    #[doc = " The id of the cycle"]
    pub id: String,
    #[doc = " The state of Cetus (day/night)"]
    pub state: CetusState,
    #[doc = " Event times"]
    #[serde(flatten)]
    pub times: crate::EventTimes,
}
impl crate::Endpoint for Cetus {
    type Return = Self;
    type Api = Worldstate;
    const ENDPOINT: &str = "/cetusCycle";
}

#[cfg(test)]
mod test_cetus {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Cetus;
    use crate::Endpoint;

    type R = <Cetus as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/cetus.json")]
        #[mode = str]
        cetus_en: &str,
    ) {
        from_str::<R>(cetus_en).unwrap();
    }
}
