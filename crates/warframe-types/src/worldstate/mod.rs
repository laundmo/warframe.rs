//! # Models
//! All Models can be found here.
//! Some of them are queryable.
//!
//! You can query every model that implements
//! [Queryable](crate::worldstate::base::Queryable)
//! [`Client`](crate::worldstate::client::Client). # Querying...
//!
//! ### ...through the client
//! To query models through the provided client, see [Client](crate::worldstate::client::Client)
//!
//! ### ...via the [Queryable] trait
//! ```rust
//! use warframe::worldstate::{
//!     Client,
//!     Error,
//!     Language,
//!     Queryable,
//!     queryable::{
//!         Cetus,
//!         Fissure,
//!     },
//! };
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let reqwest_client = reqwest::Client::new();
//!
//!     let cetus: Cetus = Cetus::query(
//!         "https://api.warframestat.us/",
//!         &reqwest_client,
//!         Language::EN,
//!     )
//!     .await?;
//!     let fissures: Vec<Fissure> = Fissure::query(
//!         "https://api.warframestat.us/",
//!         &reqwest_client,
//!         Language::EN,
//!     )
//!     .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod base;
pub use base::{
    EventTimes,
    Opposite,
    TimedEvent,
};

pub mod alert;
pub mod arbitration;
pub mod archon_hunt;
pub mod cambion_drift;
pub mod cetus;
pub mod construction_progress;
pub mod daily_deal;
pub mod event;
pub mod faction;
pub mod fissure;
pub mod flash_sale;
pub mod items;
// mod global_upgrades;
pub mod damage_type;
pub mod deep_archimedea;
pub mod global_upgrades;
pub mod invasion;
pub mod mission;
pub mod mission_type;
pub mod news;
pub mod nightwave;
pub mod orb_vallis;
pub mod reward;
pub mod reward_type;
pub mod sortie;
pub mod steel_path;
pub mod syndicate;
pub mod syndicate_mission;
pub mod void_trader;

pub mod queryable {
    pub use super::{
        alert::Alert,
        arbitration::Arbitration,
        archon_hunt::ArchonHunt,
        cambion_drift::CambionDrift,
        cetus::Cetus,
        construction_progress::ConstructionProgress,
        daily_deal::DailyDeal,
        deep_archimedea::DeepArchimedea,
        event::Event,
        fissure::Fissure,
        flash_sale::FlashSale,
        global_upgrades::GlobalUpgrade,
        invasion::Invasion,
        news::News,
        nightwave::Nightwave,
        orb_vallis::OrbVallis,
        sortie::Sortie,
        steel_path::SteelPath,
        syndicate::Syndicate,
        void_trader::VoidTrader,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub struct ItemStringWrapper(String);

impl ItemStringWrapper {
    #[must_use]
    pub fn inner(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for ItemStringWrapper {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
use serde::Deserialize;

use crate::{
    Api,
    Language,
};

/// The `ApiErrorResponse` struct represents an error with a string message and an error code.
/// This is "as is", meaning this is how the API returns this error.
#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{error} [CODE {code}]")]
pub struct ApiErrorResponse {
    /// The error message
    pub error: String,
    /// The status code returned
    pub code: u16,
}

pub struct Worldstate;
impl Api for Worldstate {
    type ErrorJson = ApiErrorResponse;
    const DEFAULT_ORIGIN: &str = "";
    fn request_apply_language(parts: &mut crate::HttpParts, language: Language) {
        parts.add_query("language", language);
    }
}
