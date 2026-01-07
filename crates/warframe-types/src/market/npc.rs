use serde::Deserialize;

use super::{
    i18n::I18N,
    
};



/// Represents the `/npcs` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub i18n: I18N<NpcI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NpcI18N {
    pub name: String,
    pub icon: String,
    pub thumb: String,
}

#[cfg(test)]
mod test {
    use super::Npc;
    use crate::market::{
        
        ResponseBase,
    };

    #[rstest::rstest]
    fn npc(
        #[files("src/market/fixtures/npc.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<Npc>>(json)?;

        Ok(())
    }
}
