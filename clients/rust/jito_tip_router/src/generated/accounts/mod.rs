//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

pub(crate) mod r#ballot_box;
pub(crate) mod r#base_reward_router;
pub(crate) mod r#epoch_snapshot;
pub(crate) mod r#ncn_config;
pub(crate) mod r#ncn_reward_router;
pub(crate) mod r#operator_snapshot;
pub(crate) mod r#vault_registry;
pub(crate) mod r#weight_table;

pub use self::{
    r#ballot_box::*, r#base_reward_router::*, r#epoch_snapshot::*, r#ncn_config::*,
    r#ncn_reward_router::*, r#operator_snapshot::*, r#vault_registry::*, r#weight_table::*,
};
