use serde::Deserialize;

use super::I18N;

/// Represents the `/lich/ephemeras` endpoint
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LichEphemera {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub animation: String,
    pub element: String,
    pub i18n: I18N<LichEphemeraI18N>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LichEphemeraI18N {
    pub name: String,
    pub icon: String,
    pub thumb: String,
}

#[cfg(test)]
mod test {
    use super::LichEphemera;
    use crate::market::ResponseBase;

    #[rstest::rstest]
    fn lich_ephemera(
        #[files("src/market/fixtures/lich_ephemera.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<LichEphemera>>(json)?;

        Ok(())
    }
}
