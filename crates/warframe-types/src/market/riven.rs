use serde::Deserialize;

use super::{
    I18N,
    riven_group::RivenGroup,
    riven_type::RivenType,
};

/// Represents the `/riven/weapons` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct Riven {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub group: Option<RivenGroup>,
    pub riven_type: Option<RivenType>,
    pub disposition: f64,
    pub req_mastery_rank: u8,
    pub i18n: I18N<RivenI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RivenI18N {
    item_name: Option<String>,
    wiki_link: Option<String>,
    icon: String,
    thumb: String,
}

#[cfg(test)]
mod test {
    use super::Riven;
    use crate::market::ResponseBase;

    #[rstest::rstest]
    fn riven(
        #[files("src/market/fixtures/riven.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<Riven>>(json)?;

        Ok(())
    }
}
