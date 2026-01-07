/// An enumeration representing various supported languages.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, PartialOrd, Eq, Ord, Hash)]
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
    /// Chinese (`ZH`)
    ZH,
    /// English (`EN`)
    EN,
    /// Ukrainian (`UK`)
    UK,
    /// Turkish (`TR`)
    TR,
    /// Japanese (`JA`)
    JA,
    /// Traditional Chinese (`TC`)
    TC,
    /// Thai (`TH`)
    TH,
}
