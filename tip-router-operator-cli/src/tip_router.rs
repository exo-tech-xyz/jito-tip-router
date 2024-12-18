use anyhow::Result;
use ellipsis_client::{ClientSubset, EllipsisClient, EllipsisClientResult};
use jito_bytemuck::AccountDeserialize;
use jito_tip_router_client::instructions::CastVoteBuilder;
use jito_tip_router_core::ncn_config::NcnConfig;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

/// Fetch and deserialize
pub async fn get_ncn_config(client: &EllipsisClient, ncn_pubkey: &Pubkey) -> Result<NcnConfig> {
    let config_pda = NcnConfig::find_program_address(&jito_tip_router_program::id(), ncn_pubkey).0;
    let config = client.get_account(&config_pda).await?;
    Ok(*NcnConfig::try_from_slice_unchecked(config.data.as_slice()).unwrap())
}

/// Generate and send a CastVote instruction with the merkle root.
pub async fn cast_vote(
    client: &EllipsisClient,
    payer: &Keypair,
    ncn: Pubkey,
    operator: Pubkey,
    operator_admin: &Keypair,
    meta_merkle_root: [u8; 32],
    ncn_epoch: u64,
) -> EllipsisClientResult<Signature> {
    let ncn_config = jito_tip_router_core::ncn_config::NcnConfig::find_program_address(
        &jito_tip_router_program::id(),
        &ncn,
    )
    .0;

    let ballot_box = jito_tip_router_core::ballot_box::BallotBox::find_program_address(
        &jito_tip_router_program::id(),
        &ncn,
        ncn_epoch,
    )
    .0;

    let epoch_snapshot = jito_tip_router_core::epoch_snapshot::EpochSnapshot::find_program_address(
        &jito_tip_router_program::id(),
        &ncn,
        ncn_epoch,
    )
    .0;

    let operator_snapshot =
        jito_tip_router_core::epoch_snapshot::OperatorSnapshot::find_program_address(
            &jito_tip_router_program::id(),
            &operator,
            &ncn,
            ncn_epoch,
        )
        .0;

    let ix = CastVoteBuilder::new()
        .ncn_config(ncn_config)
        .ballot_box(ballot_box)
        .ncn(ncn)
        .epoch_snapshot(epoch_snapshot)
        .operator_snapshot(operator_snapshot)
        .operator(operator)
        .operator_admin(operator_admin.pubkey())
        .restaking_program(jito_restaking_program::id())
        .meta_merkle_root(meta_merkle_root)
        .epoch(ncn_epoch)
        .instruction();

    let blockhash = client.get_latest_blockhash().await?;
    let tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    client
        .process_transaction(tx, &[payer, operator_admin])
        .await
}