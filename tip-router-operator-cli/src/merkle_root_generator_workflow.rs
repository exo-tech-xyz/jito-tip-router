use {
    meta_merkle_tree::{ GeneratedMerkleTreeCollection, StakeMetaCollection, MerkleRootGeneratorError },
    log::*,
    std::fmt::Debug,
    thiserror::Error,
    // ellipsis_client::EllipsisClient,
};

#[derive(Error, Debug)]

pub async fn generate_merkle_root(
    stake_meta_coll: StakeMetaCollection,
    protocol_fee_bps: u16,
) -> Result<GeneratedMerkleTreeCollection, MerkleRootGeneratorError> {
    let merkle_tree_coll = GeneratedMerkleTreeCollection::new_from_stake_meta_collection(
        stake_meta_coll,
        protocol_fee_bps,
    ).await?;

    Ok(merkle_tree_coll)
}