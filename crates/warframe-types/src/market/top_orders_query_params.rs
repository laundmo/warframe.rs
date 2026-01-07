#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TopOrdersQueryParams {
    // INFO: Slug is required anyway, so it'll be passed as an argument
    // pub slug: Slug,
    pub rank: Option<u8>,
    pub rank_lt: Option<u8>,

    pub charges: Option<u8>,
    pub charges_lt: Option<u8>,

    pub amber_stars: Option<u8>,
    pub amber_stars_lt: Option<u8>,

    pub cyan_stars: Option<u8>,
    pub cyan_stars_lt: Option<u8>,

    pub subtype: Option<String>,
}
