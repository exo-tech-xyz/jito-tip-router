pub mod cast_vote;
pub mod ledger_utils;
pub mod stake_meta_generator;
pub use crate::cli::{Cli, Commands};
pub mod cli;
pub use crate::process_epoch::process_epoch;
pub mod process_epoch;

use crate::ledger_utils::get_bank_from_ledger;
use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use jito_tip_distribution_sdk::{TipDistributionAccount, TIP_DISTRIBUTION_SEED};
use jito_tip_payment::{
    CONFIG_ACCOUNT_SEED, TIP_ACCOUNT_SEED_0, TIP_ACCOUNT_SEED_1, TIP_ACCOUNT_SEED_2,
    TIP_ACCOUNT_SEED_3, TIP_ACCOUNT_SEED_4, TIP_ACCOUNT_SEED_5, TIP_ACCOUNT_SEED_6,
    TIP_ACCOUNT_SEED_7,
};
use meta_merkle_tree::generated_merkle_tree::GeneratedMerkleTreeCollection;
use meta_merkle_tree::meta_merkle_tree::MetaMerkleTree;
use solana_sdk::{account::AccountSharedData, pubkey::Pubkey, stake_history::Epoch};
use std::path::{Path, PathBuf};

// TODO where did these come from?
pub struct TipPaymentPubkeys {
    config_pda: Pubkey,
    tip_pdas: Vec<Pubkey>,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct TipAccountConfig {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
    pub bump: u8,
}

fn derive_tip_payment_pubkeys(program_id: &Pubkey) -> TipPaymentPubkeys {
    let config_pda = Pubkey::find_program_address(&[CONFIG_ACCOUNT_SEED], program_id).0;
    let tip_pda_0 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_0], program_id).0;
    let tip_pda_1 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_1], program_id).0;
    let tip_pda_2 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_2], program_id).0;
    let tip_pda_3 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_3], program_id).0;
    let tip_pda_4 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_4], program_id).0;
    let tip_pda_5 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_5], program_id).0;
    let tip_pda_6 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_6], program_id).0;
    let tip_pda_7 = Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_7], program_id).0;

    TipPaymentPubkeys {
        config_pda,
        tip_pdas: vec![
            tip_pda_0, tip_pda_1, tip_pda_2, tip_pda_3, tip_pda_4, tip_pda_5, tip_pda_6, tip_pda_7,
        ],
    }
}

/// Convenience wrapper around [TipDistributionAccount]
pub struct TipDistributionAccountWrapper {
    pub tip_distribution_account: TipDistributionAccount,
    pub account_data: AccountSharedData,
    pub tip_distribution_pubkey: Pubkey,
}

// TODO: move to program's sdk
pub fn derive_tip_distribution_account_address(
    tip_distribution_program_id: &Pubkey,
    vote_pubkey: &Pubkey,
    epoch: Epoch,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TIP_DISTRIBUTION_SEED,
            vote_pubkey.to_bytes().as_ref(),
            epoch.to_le_bytes().as_ref(),
        ],
        tip_distribution_program_id,
    )
}

pub fn get_merkle_root(
    ledger_path: &Path,
    account_paths: Vec<PathBuf>,
    full_snapshots_path: PathBuf,
    desired_slot: &u64,
    tip_distribution_program_id: &Pubkey,
    out_path: &str,
    tip_payment_program_id: &Pubkey,
    protocol_fee_bps: u16,
) -> Result<MetaMerkleTree> {
    // Grab stake meta collection
    let stake_meta_collection = stake_meta_generator::generate_stake_meta(
        ledger_path,
        account_paths,
        full_snapshots_path,
        desired_slot,
        tip_distribution_program_id,
        out_path,
        tip_payment_program_id,
    )?;

    // Generate merkle tree collection
    let merkle_tree_coll = GeneratedMerkleTreeCollection::new_from_stake_meta_collection(
        stake_meta_collection,
        protocol_fee_bps,
    )?;

    // Convert to MetaMerkleTree
    let meta_merkle_tree =
        MetaMerkleTree::new_from_generated_merkle_tree_collection(merkle_tree_coll)?;

    Ok(meta_merkle_tree)
}
