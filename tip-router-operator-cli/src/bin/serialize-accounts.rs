use anchor_lang::prelude::*;
use base64;
use clap::Parser;
use jito_tip_distribution_sdk::TipDistributionAccount;
use serde_json::json;
use solana_program::pubkey::Pubkey;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'v', long)]
    validator_vote_account: String,

    #[arg(short = 'm', long)]
    merkle_root_upload_authority: String,

    #[arg(short = 'e', long)]
    epoch_created_at: u64,

    #[arg(short = 'c', long)]
    validator_commission_bps: u16,

    #[arg(short = 'x', long)]
    expires_at: u64,

    #[arg(short = 'b', long)]
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

    // Serialize using AnchorSerialize
    let binary_data = account.try_to_vec().expect("Failed to serialize account");

    // Encode the binary data as base64
    let base64_data = base64::encode(&binary_data);

    // Create the JSON structure
    let json_data = json!({
        "pubkey": args.validator_vote_account,
        "account": {
            "lamports": 0,  // Replace with actual lamports if available
            "data": [base64_data, "base64"],
            "owner": args.merkle_root_upload_authority,
            "executable": false,
            "rentEpoch": 0,  // Replace with actual rent epoch if available
            "space": binary_data.len()
        }
    });

    // Write the JSON data to a file
    // Use the validator_vote_account as part of the filename
    let filename = format!(
        "tests/fixtures/accounts/tip_distribution_account_{}.json",
        args.validator_vote_account
    );

    // Write the JSON data to a unique file
    let mut file = File::create(&filename).unwrap();
    file.write_all(json_data.to_string().as_bytes()).unwrap();

    println!(
        "Serialized TipDistributionAccount to JSON format in file: {}",
        filename
    );
}
