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
#[endpoint(Worldstate:"/cetusCycle" -> Self)]
pub struct Cetus {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// The id of the cycle
    pub id: String,

    /// The state of Cetus (day/night)
    pub state: CetusState,
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
