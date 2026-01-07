use crate::{
    internal_prelude::*,
    worldstate::mission_type::MissionType,
};

#[endpoint(Worldstate:"/deepArchimedea" -> Self)]
pub struct DeepArchimedea {
    /// Event times
    #[serde(flatten)]
    pub times: crate::EventTimes,

    pub id: String,
    pub missions: [DeepArchimedeaMission; 3],
    pub personal_modifiers: Vec<DeepArchimedeaModifier>,
}

#[model]
pub struct DeepArchimedeaMission {
    #[serde(rename = "mission")]
    pub r#type: MissionType,
    pub deviation: DeepArchimedeaModifier,
    pub risk_variables: Vec<DeepArchimedeaModifier>,
}

#[model]
pub struct DeepArchimedeaModifier {
    pub key: String,
    pub name: String,
    pub description: String,
}

// COMMENTED OUT FOR NOW: unsure how they're related. Sticking to String fields for now

// #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
// pub enum Deviation {
//     /// Enemy guns launch large, slow moving orbs instead of their usual ordnance.
//     ArcadeAutomata,

//     /// Drones fly above Flare, spraying them with Efervon gas.
//     ChemicalNoise,

//     /// -50% melee combo chance
//     DullBlades,

//     /// Consequences of opening a Supply Cache are active from the start and will intensify once
// it     /// is opened. Failure to open a Supply Cache doubles the kill count required to finish
// the     /// mission.
//     EscalateImmediately,

//     /// Enemies take 95% less damage from non-heavy weapons. Enemies will drop heavy ammo packs
// and     /// heavy weapon recall time reduced to 5s.
//     HeavyWarfare,

//     /// Within the underground, hostile overgrowths will attack if players stops moving.
//     HostileOvergrowth,

//     /// Jade Wisps haunt the region. If approached, they chase down the player responsible and
//     /// transform into Jade Light beams.
//     JadeSpring,

//     /// Techrot Miasmites swarm out of the shadows through the mission.
//     MiasmiteHive,

//     /// Duration of negative Status Effects is tripled.
//     OverSensitive,

//     /// Techrot enemies attempt to melee attack Hell-Scrubbers, bursting on contact and
// increasing     /// scrubber contamination by 33%.
//     TechrotConjunction,
// }

#[cfg(test)]
mod test_deep_archimedea {
    use rstest::rstest;
    use serde_json::from_str;

    use super::DeepArchimedea;
    use crate::Endpoint;

    type R = <DeepArchimedea as Endpoint>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/fixtures/deep_archimedea.json")]
        #[mode = str]
        deep_archimedea_en: &str,
    ) {
        from_str::<R>(deep_archimedea_en).unwrap();
    }
}
