use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    pub id: String,
    pub apps: VersionApps,
    pub collections: VersionCollections,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionApps {
    pub ios: String,
    pub android: String,
    pub min_ios: String,
    pub min_android: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VersionCollections {
    pub items: String,

    pub rivens: String,

    pub liches: String,

    pub sisters: String,

    pub missions: String,

    pub npcs: String,

    pub locations: String,
}

#[cfg(test)]
mod test {
    use super::Versions;
    use crate::market::ResponseBase;

    #[rstest::rstest]
    fn test_versions(
        #[files("src/market/fixtures/versions.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<Versions>>(json)?;

        Ok(())
    }
}
