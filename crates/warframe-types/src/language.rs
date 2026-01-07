use derive_more::derive::Display;
use warframe_macros::model;

#[model]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// German (`DE`)
    DE,
    /// Spanish (`ES`)
    ES,
    /// French (`FR`)
    FR,
    /// Italian (`IT`)
    IT,
    /// Korean (`KO`)
    KO,
    /// Polish (`PL`)
    PL,
    /// Portuguese (`PT`)
    PT,
    /// Russian (`RU`)
    RU,
    /// Chinese (`ZH`), zh-hans
    #[serde(alias = "zh-hans")]
    ZH,
    /// English (`EN`)
    EN,
    /// Ukrainian (`UK`)
    UK,
    /// Turkish (`TR`)
    TR,
    /// Japanese (`JA`)
    JA,
    /// Traditional Chinese (`TC`), zh-hant
    #[serde(alias = "zh-hant")]
    TC,
    /// Thai (`TH`)
    TH,
}
impl Language {
    pub fn get_market_string(&self) -> String {
        match self {
            Language::ZH => "zh-hans".to_string(),
            Language::TC => "zh-hant".to_string(),
            _ => self.to_string(),
        }
    }
}

// pub enum Language {
//     #[serde(rename = "ko")]
//     Ko,
//     #[serde(rename = "ru")]
//     Ru,
//     #[serde(rename = "de")]
//     De,
//     #[serde(rename = "fr")]
//     Fr,
//     #[serde(rename = "pt")]
//     Pt,
//     #[serde(rename = "zh-hans")]
//     ZhHans,
//     #[serde(rename = "zh-hant")]
//     ZhHant,
//     #[serde(rename = "es")]
//     Es,
//     #[serde(rename = "it")]
//     It,
//     #[serde(rename = "pl")]
//     Pl,
//     #[serde(rename = "uk")]
//     Uk,
//     #[serde(rename = "en")]
//     #[default]
//     En,
// }
