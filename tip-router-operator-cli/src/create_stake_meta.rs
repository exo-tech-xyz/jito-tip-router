use std::path::PathBuf;
use solana_ledger::{
  bank_forks_utils::{self},
  blockstore::{Blockstore, BlockstoreError},
  blockstore_options::{AccessType, BlockstoreOptions},
  blockstore_processor::{self, ProcessOptions},
};
use solana_sdk::genesis_config::GenesisConfig;


pub fn create_stake_meta(
  ledger_path: &PathBuf,
  accounts_paths: Vec<PathBuf>,
  full_snapshots_path: &PathBuf,
) {

  let genesis_config = GenesisConfig::load(ledger_path).unwrap();
    // Error handling is a modified copy pasta from ledger utils
    let blockstore = match Blockstore::open_with_options(
        ledger_path,
        BlockstoreOptions {
            access_type: AccessType::Secondary,
            ..BlockstoreOptions::default()
        },
    ) {
        Ok(blockstore) => blockstore,
        Err(BlockstoreError::RocksDb(err)) => {
            // Missing essential file, indicative of blockstore not existing
            let missing_blockstore = err
                .to_string()
                .starts_with("IO error: No such file or directory:");
            // Missing column in blockstore that is expected by software
            let missing_column = err
                .to_string()
                .starts_with("Invalid argument: Column family not found:");
            // The blockstore settings with Primary access can resolve the
            // above issues automatically, so only emit the help messages
            // if access type is Secondary
            let is_secondary = false; // access_type == AccessType::Secondary;

            if missing_blockstore && is_secondary {
                panic!(
                    "Failed to open blockstore at {ledger_path:?}, it is missing at least one \
                     critical file: {err:?}"
                );
            } else if missing_column && is_secondary {
                panic!(
                    "Failed to open blockstore at {ledger_path:?}, it does not have all necessary \
                     columns: {err:?}"
                );
            } else {
                panic!("Failed to open blockstore at {ledger_path:?}: {err:?}");
            }
        }
        Err(err) => {
            panic!("Failed to open blockstore at {ledger_path:?}: {err:?}");
        }
    };

}