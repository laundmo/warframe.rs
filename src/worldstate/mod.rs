//! # The worldstate module
//!
//! Get information about various parts of the game.
//!
//! ## Quickstart
//! ```rust,no_run
//! use warframe::worldstate::{
//!     Client,
//!     Error,
//!     queryable::{
//!         Cetus,
//!         Fissure,
//!     },
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let cetus: Cetus = client.fetch::<Cetus>().await?;
//!     let fissures: Vec<Fissure> = client.fetch::<Fissure>().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod utils;

mod listener;
mod listener_nested;

pub use client::Client;
pub use utils::Change;
/// This is a re-export of the `model` macro in case you want to use it in your own code.
/// To implement a, for example, missing model.
pub use warframe_macros::model;
use warframe_types::Language;
pub use warframe_types::worldstate::{
    ItemStringWrapper,
    archon_hunt::ArchonHuntMission,
    base::{
        Opposite,
        TimedEvent,
    },
    cambion_drift::CambionDriftState,
    cetus::CetusState,
    damage_type::{
        CombinedElementalDamage,
        DamageType,
        ElementalDamage,
        PhysicalDamage,
    },
    deep_archimedea::{
        DeepArchimedeaMission,
        DeepArchimedeaModifier,
    },
    faction::Faction,
    fissure::Tier,
    invasion::InvasionMember,
    items,
    items::Item,
    mission::Mission,
    mission_type::MissionType,
    nightwave::{
        NightwaveChallenge,
        NightwaveChallengeType,
    },
    orb_vallis::OrbVallisState,
    reward::Reward,
    reward_type::RewardType,
    sortie::SortieMission,
    steel_path::SteelPathShopItem,
    syndicate_mission::{
        SyndicateJob,
        SyndicateMission,
    },
    void_trader::VoidTraderInventoryItem,
};

use crate::Error;
type WorldstateError = Error<warframe_types::worldstate::ApiErrorResponse>;

pub trait ItemStringWrapperQuery {
    async fn query(&self, client: Client) -> Result<Option<Item>, WorldstateError>;
    async fn query_using_lang(
        &self,
        client: Client,
        language: Language,
    ) -> Result<Option<Item>, WorldstateError>;
}

impl ItemStringWrapperQuery for ItemStringWrapper {
    /// Queries an item using the provided client.
    ///
    /// This is a convenience function.
    ///
    /// # Arguments
    ///
    /// * `client` - The client used to query the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Item>` if the query is successful, or an `Error` if it
    /// fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the client fails to query the item.
    async fn query(&self, client: Client) -> Result<Option<Item>, WorldstateError> {
        client.query_item(self.inner()).await
    }

    /// Queries an item using the provided client with the provided localization
    ///
    /// This is a convenience function.
    ///
    /// # Arguments
    ///
    /// * `client` - The client used to query the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Item>` if the query is successful, or an `Error` if it
    /// fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the client fails to query the item.
    async fn query_using_lang(
        &self,
        client: Client,
        language: Language,
    ) -> Result<Option<Item>, WorldstateError> {
        client.query_item_using_lang(self.inner(), language).await
    }
}
