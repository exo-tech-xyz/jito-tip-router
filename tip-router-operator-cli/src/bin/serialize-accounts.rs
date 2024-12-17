use clap::Parser;
use jito_tip_distribution_sdk::TipDistributionAccount;
use serde_json::to_string_pretty;
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

fn main() {
    let args = Args::parse();

    let validator_vote_account =
        Pubkey::from_str(&args.validator_vote_account).expect("Invalid pubkey");
    let merkle_root_upload_authority =
        Pubkey::from_str(&args.merkle_root_upload_authority).expect("Invalid pubkey");
    // Set merkle_root to None if not provided
    let merkle_root = args.merkle_root.as_ref().map(|root| {
        let bytes = hex::decode(root).expect("Invalid hex for merkle root");
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        array
    });

    let account = TipDistributionAccount {
        validator_vote_account,
        merkle_root_upload_authority,
        merkle_root,
        epoch_created_at: args.epoch_created_at,
        validator_commission_bps: args.validator_commission_bps,
        expires_at: args.expires_at,
        bump: args.bump,
    };

    let serialized = to_string_pretty(&account).unwrap();
    let mut file = File::create("tests/fixtures/accounts/tip_distribution_account.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();

    println!("Serialized TipDistributionAccount to JSON");
}
