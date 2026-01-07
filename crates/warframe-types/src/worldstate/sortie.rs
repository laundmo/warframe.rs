use crate::internal_prelude::*;

use super::faction::Faction;

/// A Mission corresponding to a Sortie
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SortieMission {
    /// The i18n Mission type of this mission
    pub mission_type: String,

    /// The Modifier of this mission (e.g. Augmented Enemy Armor)
    pub modifier: String,

    /// The description of the modifier of this mission (e.g. Enemies have Improved/Added armor.
    /// Corrosive Projection effects are halved.)
    pub modifier_description: String,

    /// The i18n of the name
    pub node: String,
}

/// Data about the missions for the current sortie
#[endpoint(Worldstate:"/sortie" -> Self)]
pub struct Sortie {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    /// Unique identifier for this object/event/thing
    pub id: String,

    /// The name of the boss
    pub boss: String,

    /// The faction you are up against
    pub faction: Faction,

    /// The 3 missions corresponding to this sortie
    #[serde(rename = "variants")]
    pub missions: [SortieMission; 3],
}

#[cfg(test)]
mod test_sortie {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Sortie;
    use crate::Endpoint;

    type R = <Sortie as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/sortie.json")]
        #[mode = str]
        sortie_en: &str,
    ) {
        from_str::<R>(sortie_en).unwrap();
    }
}
