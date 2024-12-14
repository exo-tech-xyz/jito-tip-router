use {
    crate::{ read_json_from_file, GeneratedMerkleTreeCollection, StakeMetaCollection },
    log::*,
    solana_client::rpc_client::RpcClient,
    std::{ fmt::Debug, fs::File, io::{ BufWriter, Write }, path::PathBuf },
    thiserror::Error,
    solana_runtime::bank::Bank,
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
}

pub async fn generate_merkle_root(
    stake_meta_coll: StakeMetaCollection,
    bank: &mut Bank,
) -> Result<GeneratedMerkleTreeCollection, MerkleRootGeneratorError> {
    let merkle_tree_coll = GeneratedMerkleTreeCollection::new_from_stake_meta_collection(
        stake_meta_coll,
        bank
    ).await?;

    Ok(merkle_tree_coll)
}