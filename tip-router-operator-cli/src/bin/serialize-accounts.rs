use clap::Parser;
// use jito_tip_distribution_sdk::TipDistributionAccount;
use jito_tip_distribution_sdk::jito_tip_distribution::types::MerkleRoot;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
/// Command-line arguments for the script
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Validator vote account
    #[arg(short, long)]
    validator_vote_account: String,

    /// Merkle root upload authority
    #[arg(short, long)]
    merkle_root_upload_authority: String,

    /// Merkle root (optional)
    #[arg(short, long)]
    merkle_root: Option<String>,

    /// Epoch created at
    #[arg(short, long)]
    epoch_created_at: u64,

    /// Validator commission basis points
    #[arg(short, long)]
    validator_commission_bps: u16,

    /// Expires at
    #[arg(short, long)]
    expires_at: u64,

    /// Bump
    #[arg(short, long)]
    bump: u8,
}

#[derive(Serialize, Deserialize)]
struct TipDistributionAccount {
    validator_vote_account: Pubkey,
    merkle_root_upload_authority: Pubkey,
    merkle_root: Option<[u8; 32]>, // Assuming MerkleRoot is a 32-byte array
    epoch_created_at: u64,
    validator_commission_bps: u16,
    expires_at: u64,
    bump: u8,
}
fn main() {
    let args = Args::parse();

    let validator_vote_account =
        Pubkey::from_str(&args.validator_vote_account).expect("Invalid pubkey");
    let merkle_root_upload_authority =
        Pubkey::from_str(&args.merkle_root_upload_authority).expect("Invalid pubkey");

    let account = TipDistributionAccount {
        validator_vote_account,
        merkle_root_upload_authority,
        merkle_root: None,
        epoch_created_at: args.epoch_created_at,
        validator_commission_bps: args.validator_commission_bps,
        expires_at: args.expires_at,
        bump: args.bump,
    };

    // Serialize the account to binary
    let binary_data = bincode::serialize(&account).expect("Failed to serialize account");

    // Encode the binary data as base64
    let base64_data = base64::encode(&binary_data);

    // Write the base64 data to a file
    let mut file = File::create("tests/fixtures/accounts/tip_distribution_account.json").unwrap();
    file.write_all(base64_data.as_bytes()).unwrap();

    println!("Serialized TipDistributionAccount to base64");
}
