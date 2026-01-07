use serde::{
    Deserialize,
    Deserializer,
};

use super::{
    base::TimedEvent,
    faction::Faction,
    mission_type::MissionType,
};
use crate::internal_prelude::*;

fn deserialize_expiry<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    Ok(Some(
        chrono::DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .or_else(|err| {
                if matches!(
                    err.kind(),
                    chrono::format::ParseErrorKind::OutOfRange
                        | chrono::format::ParseErrorKind::Invalid
                ) {
                    Ok(DateTime::MAX_UTC)
                } else {
                    Err(serde::de::Error::custom(err.to_string()))
                }
            })?,
    ))
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EventTimes {
    activation: Option<DateTime>,
    #[serde[deserialize_with = "deserialize_expiry"]]
    expiry: Option<DateTime>,
}

/// Information about an arbitration
#[endpoint(Worldstate:"/arbitration" -> Self)]
pub struct Arbitration {
    /// Event times
    #[serde(flatten)]
    pub times: EventTimes,

    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,

    /// The i18n faction you are up against
    #[serde(rename(deserialize = "enemy"))]
    pub faction: Faction,

    /// The faction you are up against
    #[serde(rename(deserialize = "enemyKey"))]
    pub faction_key: Option<Faction>,

    /// The i18n type of the mission
    #[serde(rename(deserialize = "type"))]
    pub mission_type: String,

    /// The type of the mission
    #[serde(rename(deserialize = "typeKey"))]
    pub mission_type_key: MissionType,

    /// Whether this mission requires archwing
    pub archwing: bool,

    /// Whether this mission requires sharkwing
    pub sharkwing: bool,
}

impl Arbitration {
    /// Whether the arbitration is still valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.times.expiry.is_some_and(|dt| dt != DateTime::MAX_UTC)
    }
}

#[cfg(test)]
mod test_arbitration {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Arbitration;
    use crate::Endpoint;

    type R = <Arbitration as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/arbitration.json")]
        #[mode = str]
        arbitration_en: &str,
    ) {
        from_str::<R>(arbitration_en).unwrap();
    }
}
