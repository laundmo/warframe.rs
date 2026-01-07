use serde::Deserialize;

use super::i18n::I18N;

/// Represents the `/lich/quirks` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LichQuirk {
    pub id: String,
    pub slug: String,
    pub group: Option<String>,
    pub i18n: I18N<LichQuirkI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LichQuirkI18N {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub thumb: Option<String>,
}

#[cfg(test)]
mod test {
    use super::LichQuirk;
    use crate::market::ResponseBase;

    #[rstest::rstest]
    fn lich_quirk(
        #[files("src/market/fixtures/lich_quirk.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<LichQuirk>>(json)?;

        Ok(())
    }
}
