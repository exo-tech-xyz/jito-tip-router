pub mod tip_router;
pub use crate::cli::{Cli, Commands};
pub mod cli;
pub use crate::process_epoch::process_epoch;
pub mod process_epoch;

use meta_merkle_tree::{generated_merkle_tree::GeneratedMerkleTreeCollection, meta_merkle_tree::MetaMerkleTree};

pub fn get_meta_merkle_root(
  ledger_path: &Path,
  account_paths: Vec<PathBuf>,
  full_snapshots_path: PathBuf,
  desired_slot: &Slot,
  tip_distribution_program_id: &Pubkey,
  out_path: &str,
  tip_payment_program_id: &Pubkey,
  protocol_fee_bps: u64,
  snapshots_enabled: bool,
) -> std::result::Result<MetaMerkleTree, MerkleRootError> {
  // Get stake meta collection
  let stake_meta_collection = stake_meta_generator::generate_stake_meta(
      ledger_path,
      account_paths,
      full_snapshots_path,
      desired_slot,
      tip_distribution_program_id,
      out_path,
      tip_payment_program_id,
      snapshots_enabled,
  )
  .map_err(|_| MerkleRootError::StakeMetaGeneratorError("Failed to generate stake meta"))?;

  // Generate merkle tree collection
  let merkle_tree_coll = GeneratedMerkleTreeCollection::new_from_stake_meta_collection(
      stake_meta_collection,
      protocol_fee_bps,
  )
  .map_err(|_| {
      MerkleRootError::MerkleRootGeneratorError("Failed to generate merkle tree collection")
  })?;

  // Convert to MetaMerkleTree
  let meta_merkle_tree = MetaMerkleTree::new_from_generated_merkle_tree_collection(
      merkle_tree_coll,
  )
  .map_err(|e| {
      info!("Meta merkle tree creation error: {:?}", e);
      MerkleRootError::MerkleTreeError("Failed to create meta merkle tree")
  })?;
  Ok(meta_merkle_tree)
}