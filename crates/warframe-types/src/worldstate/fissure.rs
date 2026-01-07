use super::{
    faction::Faction,
    mission_type::MissionType,
};
use crate::internal_prelude::*;

/// Represents Relic tiers
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Tier {
    /// Lith
    Lith = 1,
    /// Meso
    Meso = 2,
    /// Neo
    Neo = 3,
    /// Axi
    Axi = 4,
    /// Requiem
    Requiem = 5,
    /// Omnia
    Omnia = 6,
}

/// A Fissure Mission in which you can crack Void Relics
#[endpoint(Worldstate:"/fissures" -> Vec<Self>)]
pub struct Fissure {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// The id of the fissure
    pub id: String,

    /// The i18n of the mission
    pub mission_type: String,

    /// The type of the mission
    pub mission_type_key: MissionType,

    /// The i18n of the node
    pub node: String,

    /// The name of the node
    pub node_key: String,

    /// The tier i18n of the relic
    #[serde(rename = "tier")]
    pub tier_name: String,

    /// The Tier of the relic
    #[serde(rename = "tierNum")]
    pub tier: Tier,

    /// The i18n name of the enemy
    pub enemy: String,

    /// The type of the enemy
    #[serde(rename = "enemyKey")]
    pub faction: Faction,

    /// Whether the fissure is a storm
    pub is_storm: bool,

    /// Whether the the fissure is hard (Steel Path)
    pub is_hard: bool,
}

#[cfg(test)]
mod test_fissure {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Fissure;
    use crate::Endpoint;

    type R = <Fissure as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/fissure.json")]
        #[mode = str]
        fissure_en: &str,
    ) {
        from_str::<R>(fissure_en).unwrap();
    }
}
