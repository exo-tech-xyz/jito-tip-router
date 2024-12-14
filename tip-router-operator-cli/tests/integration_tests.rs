pub mod snapshot_creator;

use {
    meta_merkle_tree::{
        meta_merkle_tree::MetaMerkleTree,
        generated_merkle_tree::GeneratedMerkleTreeCollection as MetaMerkleTreeCollection,
    },
    solana_program_test::*,
    solana_program::stake::state::StakeStateV2,
    solana_sdk::{
        signature::{ Keypair, Signer },
        pubkey::Pubkey,
        system_instruction,
        transaction::Transaction,
    },
    std::{ fs::{ self, File }, io::BufReader, sync::Arc, time::Duration, path::PathBuf },
    tempfile::TempDir,
    tip_router_operator_cli::{
        StakeMetaCollection,
        StakeMeta,
        TipDistributionMeta,
        Delegation,
        GeneratedMerkleTreeCollection as TipRouterMerkleTreeCollection,
        claim_mev_workflow,
        merkle_root_generator_workflow,
        merkle_root_upload_workflow,
        Cli,
        Commands,
        process_epoch,
        TipAccountConfig,
    },
    jito_tip_distribution::{ self, ID as TIP_DISTRIBUTION_ID },
    jito_tip_payment::{ self, ID as TIP_PAYMENT_ID },
    ellipsis_client::EllipsisClient,
    solana_client::rpc_client::RpcClient,
    solana_sdk::genesis_config::GenesisConfig,
    self::snapshot_creator::MockSnapshotCreator,
    solana_program::stake::state::StakeState,
    thiserror::Error,
    solana_sdk::account::{ Account, AccountSharedData },
    jito_tip_distribution::state::Config,
    anchor_lang::prelude::AnchorSerialize,
};

struct TestContext {
    pub context: ProgramTestContext,
    pub tip_distribution_program_id: Pubkey,
    pub tip_payment_program_id: Pubkey,
    pub payer: Keypair,
    pub stake_accounts: Vec<Keypair>, // Changed from single stake_account
    pub vote_account: Keypair,
    pub temp_dir: TempDir,
    pub output_dir: PathBuf,
}

impl TestContext {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&output_dir)?;

        let mut program_test = ProgramTest::default();

        // Add programs to test environment
        program_test.add_program("jito_tip_distribution", TIP_DISTRIBUTION_ID, None);
        program_test.add_program("jito_tip_payment", TIP_PAYMENT_ID, None);

        let mut context = program_test.start_with_context().await;
        let payer = Keypair::new();
        let vote_account = Keypair::new();

        // Fund payer account
        let tx = Transaction::new_signed_with_payer(
            &[
                system_instruction::transfer(
                    &context.payer.pubkey(),
                    &payer.pubkey(),
                    10_000_000_000 // Increased balance for multiple accounts
                ),
            ],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash
        );
        context.banks_client.process_transaction(tx).await?;

        // Create multiple stake accounts
        let stake_accounts = vec![Keypair::new(), Keypair::new(), Keypair::new()];

        // Get rent and space requirements
        let rent = context.banks_client.get_rent().await?;
        let stake_space = std::mem::size_of::<StakeStateV2>();
        let stake_rent = rent.minimum_balance(stake_space);

        // Initialize each stake account
        for stake_account in stake_accounts.iter() {
            let tx = Transaction::new_signed_with_payer(
                &[
                    system_instruction::create_account(
                        &payer.pubkey(),
                        &stake_account.pubkey(),
                        stake_rent,
                        stake_space as u64,
                        &solana_program::stake::program::id()
                    ),
                    solana_program::stake::instruction::initialize(
                        &stake_account.pubkey(),
                        &(solana_sdk::stake::state::Authorized {
                            staker: payer.pubkey(),
                            withdrawer: payer.pubkey(),
                        }),
                        &solana_sdk::stake::state::Lockup::default()
                    ),
                ],
                Some(&payer.pubkey()),
                &[&payer, stake_account],
                context.last_blockhash
            );
            context.banks_client.process_transaction(tx).await?;

            // Update blockhash between transactions
            context.last_blockhash = context.banks_client.get_latest_blockhash().await?;
        }

        // Create and initialize vote account (if needed)
        // Add vote account initialization here if required

        Ok(Self {
            context,
            tip_distribution_program_id: TIP_DISTRIBUTION_ID,
            tip_payment_program_id: TIP_PAYMENT_ID,
            payer,
            stake_accounts, // Store all stake accounts instead of just one
            vote_account,
            temp_dir,
            output_dir,
        })
    }

    fn create_test_stake_meta(
        &self,
        total_tips: u64,
        validator_fee_bps: u16
    ) -> StakeMetaCollection {
        let stake_meta = StakeMeta {
            validator_vote_account: self.vote_account.pubkey(),
            validator_node_pubkey: self.stake_accounts[0].pubkey(),
            maybe_tip_distribution_meta: Some(TipDistributionMeta {
                total_tips,
                merkle_root_upload_authority: self.payer.pubkey(),
                tip_distribution_pubkey: self.tip_distribution_program_id,
                validator_fee_bps,
            }),
            delegations: vec![Delegation {
                stake_account_pubkey: self.stake_accounts[0].pubkey(),
                staker_pubkey: self.payer.pubkey(),
                withdrawer_pubkey: self.payer.pubkey(),
                lamports_delegated: 1_000_000,
            }],
            total_delegated: 1_000_000,
            commission: 10,
        };

        StakeMetaCollection {
            epoch: 0,
            stake_metas: vec![stake_meta],
            bank_hash: "test_bank_hash".to_string(),
            slot: 0,
            tip_distribution_program_id: self.tip_distribution_program_id,
        }
    }

    async fn advance_clock(&mut self, slots: u64) -> Result<(), Box<dyn std::error::Error>> {
        let current_slot = self.context.banks_client.get_root_slot().await?;
        self.context.warp_to_slot(current_slot + slots)?;
        self.context.last_blockhash = self.context.banks_client.get_latest_blockhash().await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum MerkleTreeTestError {
    #[error(transparent)] ProgramTestError(#[from] ProgramTestError),

    #[error(transparent)] IoError(#[from] std::io::Error),

    #[error(transparent)] BanksClientError(#[from] BanksClientError),

    #[error(transparent)] MerkleRootGeneratorError(
        #[from] merkle_root_generator_workflow::MerkleRootGeneratorError,
    ), // Add this

    #[error("Other error: {0}")] Other(Box<dyn std::error::Error>),
}

#[tokio::test]
async fn test_merkle_tree_generation() -> Result<(), MerkleTreeTestError> {
    // Constants
    const PROTOCOL_FEE_BPS: u16 = 500;
    const VALIDATOR_FEE_BPS: u16 = 1000;
    const TOTAL_TIPS: u64 = 1_000_000;

    let mut test_context = TestContext::new().await.map_err(|e| MerkleTreeTestError::Other(e))?;

    // Get config PDA
    let (config_pda, bump) = Pubkey::find_program_address(&[b"config"], &TIP_DISTRIBUTION_ID);

    // Create config account with protocol fee
    let config = TipAccountConfig {
        authority: test_context.payer.pubkey(),
        protocol_fee_bps: PROTOCOL_FEE_BPS, // 5% protocol fee
        bump,
    };

    // Create config account
    let space = 32 + 2 + 1; // pubkey (32) + u16 (2) + u8 (1)
    let rent = test_context.context.banks_client.get_rent().await?;

    // Create account data
    let account = AccountSharedData::new(rent.minimum_balance(space), space, &TIP_DISTRIBUTION_ID);
    
    let mut config_data = vec![0u8; space];
    config.serialize(&mut config_data);

    // Create account with data
    let mut account = account;
    account.set_data(config_data);

    // Set the account
    test_context.context.set_account(&config_pda, &account);

    let stake_meta_collection = test_context.create_test_stake_meta(TOTAL_TIPS, VALIDATOR_FEE_BPS);

    let protocol_fee_amount = (((TOTAL_TIPS as u128) * (PROTOCOL_FEE_BPS as u128)) /
        10000u128) as u64;
    let validator_fee_amount = (((TOTAL_TIPS as u128) * (VALIDATOR_FEE_BPS as u128)) /
        10000u128) as u64;
    let remaining_tips = TOTAL_TIPS - protocol_fee_amount - validator_fee_amount;

    // Then use it in generate_merkle_root
    let merkle_tree_coll = merkle_root_generator_workflow::generate_merkle_root(
        stake_meta_collection.clone(),
        PROTOCOL_FEE_BPS
    ).await?;

    let generated_tree = &merkle_tree_coll.generated_merkle_trees[0];
    let nodes = &generated_tree.tree_nodes;

    // Get the protocol fee recipient PDA - use the same derivation as in the implementation
    let (protocol_fee_recipient, _) = Pubkey::find_program_address(
        &[b"protocol_fee", &(0u64).to_le_bytes()],
        &TIP_DISTRIBUTION_ID
    );

    let protocol_fee_node = nodes
        .iter()
        .find(|node| node.claimant == protocol_fee_recipient)
        .expect("Protocol fee node should exist");
    assert_eq!(protocol_fee_node.amount, protocol_fee_amount);

    // Verify validator fee node
    let validator_fee_node = nodes
        .iter()
        .find(|node| node.claimant == stake_meta_collection.stake_metas[0].validator_node_pubkey)
        .expect("Validator fee node should exist");
    assert_eq!(validator_fee_node.amount, validator_fee_amount);

    // Verify delegator nodes
    for delegation in &stake_meta_collection.stake_metas[0].delegations {
        let delegator_share = (((remaining_tips as u128) *
            (delegation.lamports_delegated as u128)) /
            (stake_meta_collection.stake_metas[0].total_delegated as u128)) as u64;

        let delegator_node = nodes
            .iter()
            .find(|node| node.claimant == delegation.staker_pubkey)
            .expect("Delegator node should exist");
        assert_eq!(
            delegator_node.amount,
            delegator_share,
            "Delegator share mismatch for stake amount {}",
            delegation.lamports_delegated
        );
    }

    // Verify node structure
    for node in nodes {
        assert_ne!(node.claimant, Pubkey::default(), "Node claimant should not be default");
        assert_ne!(
            node.claim_status_pubkey,
            Pubkey::default(),
            "Node claim status should not be default"
        );
        assert!(node.amount > 0, "Node amount should be greater than 0");
        assert!(node.proof.is_some(), "Node should have a proof");
    }

    Ok(())
}
