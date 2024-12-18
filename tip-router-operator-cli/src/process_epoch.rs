use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use ellipsis_client::EllipsisClient;
use log::info;
use meta_merkle_tree::{
    generated_merkle_tree::GeneratedMerkleTreeCollection, meta_merkle_tree::MetaMerkleTree,
};
use solana_metrics::datapoint_info;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::keypair::{read_keypair_file, Keypair},
};

use crate::{stake_meta_generator, tip_router::get_ncn_config, Cli};

pub async fn wait_for_next_epoch(rpc_client: &RpcClient) -> Result<()> {
    let current_epoch = rpc_client.get_epoch_info()?.epoch;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await; // Check every 10 seconds
        let new_epoch = rpc_client.get_epoch_info()?.epoch;

        if new_epoch > current_epoch {
            info!("New epoch detected: {} -> {}", current_epoch, new_epoch);
            return Ok(());
        }
    }
}

pub async fn get_previous_epoch_last_slot(rpc_client: &RpcClient) -> Result<u64> {
    let epoch_info = rpc_client.get_epoch_info()?;
    let current_slot = epoch_info.absolute_slot;
    let slot_index = epoch_info.slot_index;

    // Handle case where we're in the first epoch
    if current_slot < slot_index {
        return Ok(0);
    }

    let epoch_start_slot = current_slot - slot_index;
    let previous_epoch_final_slot = epoch_start_slot.saturating_sub(1);

    Ok(previous_epoch_final_slot)
}

pub async fn process_epoch(
    client: &EllipsisClient,
    previous_epoch_slot: u64,
    payer: &Keypair,
    tip_distribution_program_id: &Pubkey,
    tip_payment_program_id: &Pubkey,
    ncn_address: &Pubkey,
) -> Result<()> {
    // TODO Get the protocol fees
    let ncn_config = get_ncn_config(client, ncn_address).await.unwrap();

    // TODO Generate merkle root from ledger
    // let meta_merkle_tree = get_merkle_root(
    //     ledger_path,
    //     account_paths,
    //     full_snapshots_path,
    //     desired_slot,
    //     tip_distribution_program_id,
    //     out_path,
    //     tip_payment_program_id,
    //     PROTOCOL_FEE_BPS,
    // )
    // .unwrap();

    // TODO cast vote using the generated merkle root
    info!("Successfully completed all steps for epoch processing");
    Ok(())
}
