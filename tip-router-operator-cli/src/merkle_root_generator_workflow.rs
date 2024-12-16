use {
    crate::{ GeneratedMerkleTreeCollection, StakeMetaCollection },
    log::*,
    std::{ fmt::Debug },
    thiserror::Error,
    // ellipsis_client::EllipsisClient,
};

#[derive(Error, Debug)]
pub enum MerkleRootGeneratorError {
    #[error("Account not found")]
    AccountNotFound,
    #[error("Deserialization error")]
    DeserializationError,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    RpcError(#[from] Box<solana_client::client_error::ClientError>),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Checked math error")]
    CheckedMathError,
}

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