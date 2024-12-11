mod set_merkle_root {
    use jito_tip_distribution_sdk::{
        derive_claim_status_account_address, derive_tip_distribution_account_address,
        jito_tip_distribution,
    };
    use jito_tip_router_core::{
        ballot_box::{Ballot, BallotBox},
        error::TipRouterError,
        ncn_config::NcnConfig,
    };
    use meta_merkle_tree::{
        generated_merkle_tree::{
            self, Delegation, GeneratedMerkleTree, GeneratedMerkleTreeCollection, StakeMeta,
            StakeMetaCollection, TipDistributionMeta,
        },
        meta_merkle_tree::MetaMerkleTree,
    };
    use solana_sdk::{epoch_schedule::EpochSchedule, pubkey::Pubkey, signer::Signer};

    use crate::{
        fixtures::{
            test_builder::TestBuilder, tip_router_client::assert_tip_router_error, TestError,
            TestResult,
        },
        helpers::ballot_box::serialized_ballot_box_account,
    };

    struct GeneratedMerkleTreeCollectionFixture {
        _test_generated_merkle_tree: GeneratedMerkleTree,
        collection: GeneratedMerkleTreeCollection,
    }

    fn _create_tree_node(
        claimant_staker_withdrawer: Pubkey,
        amount: u64,
        epoch: u64,
    ) -> generated_merkle_tree::TreeNode {
        let (claim_status_pubkey, claim_status_bump) = derive_claim_status_account_address(
            &jito_tip_distribution::ID,
            &claimant_staker_withdrawer,
            &derive_tip_distribution_account_address(
                &jito_tip_distribution::ID,
                &claimant_staker_withdrawer,
                epoch,
            )
            .0,
        );

        generated_merkle_tree::TreeNode {
            claimant: claimant_staker_withdrawer,
            claim_status_pubkey,
            claim_status_bump,
            staker_pubkey: claimant_staker_withdrawer,
            withdrawer_pubkey: claimant_staker_withdrawer,
            amount,
            proof: None,
        }
    }

    fn create_generated_merkle_tree_collection(
        vote_account: Pubkey,
        merkle_root_upload_authority: Pubkey,
        epoch: u64,
    ) -> TestResult<GeneratedMerkleTreeCollectionFixture> {
        let claimant_staker_withdrawer = Pubkey::new_unique();

        let test_delegation = Delegation {
            stake_account_pubkey: claimant_staker_withdrawer,
            staker_pubkey: claimant_staker_withdrawer,
            withdrawer_pubkey: claimant_staker_withdrawer,
            lamports_delegated: 50,
        };

        let vote_account_stake_meta = StakeMeta {
            validator_vote_account: vote_account,
            validator_node_pubkey: Pubkey::new_unique(),
            maybe_tip_distribution_meta: Some(TipDistributionMeta {
                merkle_root_upload_authority,
                tip_distribution_pubkey: derive_tip_distribution_account_address(
                    &jito_tip_distribution::ID,
                    &vote_account,
                    epoch,
                )
                .0,
                total_tips: 50,
                validator_fee_bps: 0,
            }),
            delegations: vec![test_delegation.clone()],
            total_delegated: 50,
            commission: 0,
        };

        let other_validator = Pubkey::new_unique();
        let other_stake_meta = StakeMeta {
            validator_vote_account: other_validator,
            validator_node_pubkey: Pubkey::new_unique(),
            maybe_tip_distribution_meta: Some(TipDistributionMeta {
                merkle_root_upload_authority: other_validator,
                tip_distribution_pubkey: derive_tip_distribution_account_address(
                    &jito_tip_distribution::ID,
                    &other_validator,
                    epoch,
                )
                .0,
                total_tips: 50,
                validator_fee_bps: 0,
            }),
            delegations: vec![test_delegation],
            total_delegated: 50,
            commission: 0,
        };

        let stake_meta_collection = StakeMetaCollection {
            stake_metas: vec![vote_account_stake_meta, other_stake_meta],
            tip_distribution_program_id: Pubkey::new_unique(),
            bank_hash: String::default(),
            epoch,
            slot: 0,
        };

        let collection =
            GeneratedMerkleTreeCollection::new_from_stake_meta_collection(stake_meta_collection)
                .map_err(TestError::from)?;

        let test_tip_distribution_account = derive_tip_distribution_account_address(
            &jito_tip_distribution::ID,
            &vote_account,
            epoch,
        )
        .0;
        let test_generated_merkle_tree = collection
            .generated_merkle_trees
            .iter()
            .find(|tree| tree.tip_distribution_account == test_tip_distribution_account)
            .unwrap();

        Ok(GeneratedMerkleTreeCollectionFixture {
            _test_generated_merkle_tree: test_generated_merkle_tree.clone(),
            collection,
        })
    }

    struct MetaMerkleTreeFixture {
        // Contains the individual validator's merkle trees, with the TreeNode idata needed to invoke the set_merkle_root instruction (root, max_num_nodes, max_total_claim)
        _generated_merkle_tree_fixture: GeneratedMerkleTreeCollectionFixture,
        // Contains meta merkle tree with the root that all validators vote on, and proofs needed to verify the input data
        pub meta_merkle_tree: MetaMerkleTree,
    }

    fn create_meta_merkle_tree(
        vote_account: Pubkey,
        merkle_root_upload_authority: Pubkey,
        epoch: u64,
    ) -> TestResult<MetaMerkleTreeFixture> {
        let generated_merkle_tree_fixture = create_generated_merkle_tree_collection(
            vote_account,
            merkle_root_upload_authority,
            epoch,
        )
        .map_err(TestError::from)?;

        let meta_merkle_tree = MetaMerkleTree::new_from_generated_merkle_tree_collection(
            generated_merkle_tree_fixture.collection.clone(),
        )?;

        Ok(MetaMerkleTreeFixture {
            _generated_merkle_tree_fixture: generated_merkle_tree_fixture,
            meta_merkle_tree,
        })
    }

    #[tokio::test]
    async fn test_set_merkle_root_ok() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let mut tip_distribution_client = fixture.tip_distribution_client();

        let test_ncn = fixture.create_test_ncn().await?;
        let ncn_address = test_ncn.ncn_root.ncn_pubkey;
        let ncn_config_address =
            NcnConfig::find_program_address(&jito_tip_router_program::id(), &ncn_address).0;

        let epoch = 0;
        tip_distribution_client
            .do_initialize(ncn_config_address)
            .await?;
        let vote_keypair = tip_distribution_client.setup_vote_account().await?;
        let vote_account = vote_keypair.pubkey();

        tip_distribution_client
            .do_initialize_tip_distribution_account(ncn_config_address, vote_keypair, epoch, 100)
            .await?;

        let meta_merkle_tree_fixture =
            create_meta_merkle_tree(vote_account, ncn_config_address, epoch)?;
        let winning_root = meta_merkle_tree_fixture.meta_merkle_tree.merkle_root;

        let (ballot_box_address, bump, _) =
            BallotBox::find_program_address(&jito_tip_router_program::id(), &ncn_address, epoch);

        let ballot_box_fixture = {
            let mut ballot_box = BallotBox::new(ncn_address, epoch, bump, 0);
            let winning_ballot = Ballot::new(winning_root);
            ballot_box.set_winning_ballot(winning_ballot);
            ballot_box
        };

        let epoch_schedule: EpochSchedule = fixture.epoch_schedule().await;

        // Must warp before .set_account
        fixture
            .warp_slot_incremental(epoch_schedule.get_slots_in_epoch(epoch))
            .await?;

        fixture
            .set_account(
                ballot_box_address,
                serialized_ballot_box_account(&ballot_box_fixture),
            )
            .await;

        let tip_distribution_address = derive_tip_distribution_account_address(
            &jito_tip_distribution::ID,
            &vote_account,
            epoch,
        )
        .0;

        // Get proof for vote_account
        let node = meta_merkle_tree_fixture
            .meta_merkle_tree
            .get_node(&tip_distribution_address);
        let proof = node.proof.clone().unwrap();

        ballot_box_fixture
            .verify_merkle_root(
                tip_distribution_address,
                node.proof.unwrap(),
                node.validator_merkle_root,
                node.max_total_claim,
                node.max_num_nodes,
            )
            .unwrap();

        // Test wrong proof
        let res = tip_router_client
            .do_set_merkle_root(
                ncn_address,
                vote_account,
                vec![[1; 32]],
                node.validator_merkle_root,
                node.max_total_claim,
                node.max_num_nodes,
                epoch,
            )
            .await;
        assert_tip_router_error(res, TipRouterError::InvalidMerkleProof);

        // Invoke set_merkle_root
        tip_router_client
            .do_set_merkle_root(
                ncn_address,
                vote_account,
                proof,
                node.validator_merkle_root,
                node.max_total_claim,
                node.max_num_nodes,
                epoch,
            )
            .await?;

        // Fetch the tip distribution account and check root
        let tip_distribution_account = tip_distribution_client
            .get_tip_distribution_account(vote_account, epoch)
            .await?;

        let merkle_root = tip_distribution_account.merkle_root.unwrap();

        assert_eq!(merkle_root.root, node.validator_merkle_root);
        assert_eq!(merkle_root.max_num_nodes, node.max_num_nodes);
        assert_eq!(merkle_root.max_total_claim, node.max_total_claim);

        Ok(())
    }

    // TODO update to use this test once snapshot instructions work with BPF
    #[ignore]
    #[tokio::test]
    async fn _test_set_merkle_root_no_fixture() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let mut tip_distribution_client = fixture.tip_distribution_client();

        let test_ncn = fixture.create_initial_test_ncn(1, 1, None).await?;

        ///// TipRouter Setup /////
        fixture.warp_slot_incremental(1000).await?;
        fixture.snapshot_test_ncn(&test_ncn).await?;

        let clock = fixture.clock().await;
        let slot = clock.slot;
        let restaking_config_account = tip_router_client.get_restaking_config().await?;
        let ncn_epoch = slot / restaking_config_account.epoch_length();

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let ncn_config_address =
            NcnConfig::find_program_address(&jito_tip_router_program::id(), &ncn).0;

        let epoch: u64 = 0;
        tip_distribution_client
            .do_initialize(ncn_config_address)
            .await?;
        let vote_keypair = tip_distribution_client.setup_vote_account().await?;
        let vote_account = vote_keypair.pubkey();

        tip_distribution_client
            .do_initialize_tip_distribution_account(
                ncn_config_address,
                vote_keypair,
                ncn_epoch,
                100,
            )
            .await?;

        let meta_merkle_tree_fixture =
            create_meta_merkle_tree(vote_account, ncn_config_address, epoch)?;
        let winning_root = meta_merkle_tree_fixture.meta_merkle_tree.merkle_root;

        let operator = test_ncn.operators[0].operator_pubkey;
        let operator_admin = &test_ncn.operators[0].operator_admin;

        tip_router_client
            .do_cast_vote(ncn, operator, operator_admin, winning_root, ncn_epoch)
            .await?;
        let tip_distribution_address = derive_tip_distribution_account_address(
            &jito_tip_distribution::ID,
            &vote_account,
            epoch,
        )
        .0;

        // Get proof for vote_account
        let node = meta_merkle_tree_fixture
            .meta_merkle_tree
            .get_node(&tip_distribution_address);
        let proof = node.proof.clone().unwrap();

        let ballot_box = tip_router_client.get_ballot_box(ncn, epoch).await?;

        ballot_box
            .verify_merkle_root(
                tip_distribution_address,
                node.proof.unwrap(),
                node.validator_merkle_root,
                node.max_total_claim,
                node.max_num_nodes,
            )
            .unwrap();

        // Invoke set_merkle_root
        tip_router_client
            .do_set_merkle_root(
                ncn,
                vote_account,
                proof,
                node.validator_merkle_root,
                node.max_total_claim,
                node.max_num_nodes,
                epoch,
            )
            .await?;

        // Fetch the tip distribution account and check root
        let tip_distribution_account = tip_distribution_client
            .get_tip_distribution_account(vote_account, epoch)
            .await?;

        let merkle_root = tip_distribution_account.merkle_root.unwrap();

        assert_eq!(merkle_root.root, node.validator_merkle_root);
        assert_eq!(merkle_root.max_num_nodes, node.max_num_nodes);
        assert_eq!(merkle_root.max_total_claim, node.max_total_claim);

        Ok(())
    }

    #[tokio::test]
    async fn test_set_merkle_root_before_consensus() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let mut tip_distribution_client = fixture.tip_distribution_client();

        let test_ncn = fixture.create_test_ncn().await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let ncn_config_address =
            NcnConfig::find_program_address(&jito_tip_router_program::id(), &ncn).0;

        let clock = fixture.clock().await;
        let slot = clock.slot;
        let restaking_config_account = tip_router_client.get_restaking_config().await?;
        let ncn_epoch = slot / restaking_config_account.epoch_length();

        tip_distribution_client
            .do_initialize(ncn_config_address)
            .await?;
        let vote_keypair = tip_distribution_client.setup_vote_account().await?;
        let vote_account = vote_keypair.pubkey();

        tip_distribution_client
            .do_initialize_tip_distribution_account(
                ncn_config_address,
                vote_keypair,
                ncn_epoch,
                100,
            )
            .await?;

        let meta_merkle_tree_fixture =
            create_meta_merkle_tree(vote_account, ncn_config_address, ncn_epoch)?;

        let tip_distribution_address = derive_tip_distribution_account_address(
            &jito_tip_distribution::ID,
            &vote_account,
            ncn_epoch,
        )
        .0;
        let node = meta_merkle_tree_fixture
            .meta_merkle_tree
            .get_node(&tip_distribution_address);
        let proof = node.proof.clone().unwrap();

        // Initialize ballot box
        tip_router_client
            .do_initialize_ballot_box(ncn, ncn_epoch)
            .await?;

        // Try setting merkle root before consensus
        let res = tip_router_client
            .do_set_merkle_root(
                ncn,
                vote_account,
                proof,
                node.validator_merkle_root,
                node.max_total_claim,
                node.max_num_nodes,
                ncn_epoch,
            )
            .await;

        assert_tip_router_error(res, TipRouterError::ConsensusNotReached);

        Ok(())
    }
}