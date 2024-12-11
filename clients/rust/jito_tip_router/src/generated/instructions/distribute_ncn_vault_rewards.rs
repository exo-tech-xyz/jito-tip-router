//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct DistributeNcnVaultRewards {
    pub restaking_config: solana_program::pubkey::Pubkey,

    pub ncn_config: solana_program::pubkey::Pubkey,

    pub ncn: solana_program::pubkey::Pubkey,

    pub operator: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub ncn_reward_router: solana_program::pubkey::Pubkey,
}

impl DistributeNcnVaultRewards {
    pub fn instruction(
        &self,
        args: DistributeNcnVaultRewardsInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: DistributeNcnVaultRewardsInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.restaking_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ncn_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ncn, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.operator,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.ncn_reward_router,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = DistributeNcnVaultRewardsInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::JITO_TIP_ROUTER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct DistributeNcnVaultRewardsInstructionData {
    discriminator: u8,
}

impl DistributeNcnVaultRewardsInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 17 }
    }
}

impl Default for DistributeNcnVaultRewardsInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DistributeNcnVaultRewardsInstructionArgs {
    pub ncn_fee_group: u8,
    pub first_slot_of_ncn_epoch: Option<u64>,
}

/// Instruction builder for `DistributeNcnVaultRewards`.
///
/// ### Accounts:
///
///   0. `[]` restaking_config
///   1. `[]` ncn_config
///   2. `[]` ncn
///   3. `[]` operator
///   4. `[writable]` vault
///   5. `[writable]` ncn_reward_router
#[derive(Clone, Debug, Default)]
pub struct DistributeNcnVaultRewardsBuilder {
    restaking_config: Option<solana_program::pubkey::Pubkey>,
    ncn_config: Option<solana_program::pubkey::Pubkey>,
    ncn: Option<solana_program::pubkey::Pubkey>,
    operator: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    ncn_reward_router: Option<solana_program::pubkey::Pubkey>,
    ncn_fee_group: Option<u8>,
    first_slot_of_ncn_epoch: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl DistributeNcnVaultRewardsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn restaking_config(
        &mut self,
        restaking_config: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.restaking_config = Some(restaking_config);
        self
    }
    #[inline(always)]
    pub fn ncn_config(&mut self, ncn_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ncn_config = Some(ncn_config);
        self
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn operator(&mut self, operator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.operator = Some(operator);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn ncn_reward_router(
        &mut self,
        ncn_reward_router: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.ncn_reward_router = Some(ncn_reward_router);
        self
    }
    #[inline(always)]
    pub fn ncn_fee_group(&mut self, ncn_fee_group: u8) -> &mut Self {
        self.ncn_fee_group = Some(ncn_fee_group);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn first_slot_of_ncn_epoch(&mut self, first_slot_of_ncn_epoch: u64) -> &mut Self {
        self.first_slot_of_ncn_epoch = Some(first_slot_of_ncn_epoch);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = DistributeNcnVaultRewards {
            restaking_config: self.restaking_config.expect("restaking_config is not set"),
            ncn_config: self.ncn_config.expect("ncn_config is not set"),
            ncn: self.ncn.expect("ncn is not set"),
            operator: self.operator.expect("operator is not set"),
            vault: self.vault.expect("vault is not set"),
            ncn_reward_router: self
                .ncn_reward_router
                .expect("ncn_reward_router is not set"),
        };
        let args = DistributeNcnVaultRewardsInstructionArgs {
            ncn_fee_group: self
                .ncn_fee_group
                .clone()
                .expect("ncn_fee_group is not set"),
            first_slot_of_ncn_epoch: self.first_slot_of_ncn_epoch.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `distribute_ncn_vault_rewards` CPI accounts.
pub struct DistributeNcnVaultRewardsCpiAccounts<'a, 'b> {
    pub restaking_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_reward_router: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `distribute_ncn_vault_rewards` CPI instruction.
pub struct DistributeNcnVaultRewardsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub restaking_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub operator: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_reward_router: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: DistributeNcnVaultRewardsInstructionArgs,
}

impl<'a, 'b> DistributeNcnVaultRewardsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: DistributeNcnVaultRewardsCpiAccounts<'a, 'b>,
        args: DistributeNcnVaultRewardsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            restaking_config: accounts.restaking_config,
            ncn_config: accounts.ncn_config,
            ncn: accounts.ncn,
            operator: accounts.operator,
            vault: accounts.vault,
            ncn_reward_router: accounts.ncn_reward_router,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.restaking_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ncn_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ncn.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.operator.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.ncn_reward_router.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = DistributeNcnVaultRewardsInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JITO_TIP_ROUTER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.restaking_config.clone());
        account_infos.push(self.ncn_config.clone());
        account_infos.push(self.ncn.clone());
        account_infos.push(self.operator.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.ncn_reward_router.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `DistributeNcnVaultRewards` via CPI.
///
/// ### Accounts:
///
///   0. `[]` restaking_config
///   1. `[]` ncn_config
///   2. `[]` ncn
///   3. `[]` operator
///   4. `[writable]` vault
///   5. `[writable]` ncn_reward_router
#[derive(Clone, Debug)]
pub struct DistributeNcnVaultRewardsCpiBuilder<'a, 'b> {
    instruction: Box<DistributeNcnVaultRewardsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> DistributeNcnVaultRewardsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(DistributeNcnVaultRewardsCpiBuilderInstruction {
            __program: program,
            restaking_config: None,
            ncn_config: None,
            ncn: None,
            operator: None,
            vault: None,
            ncn_reward_router: None,
            ncn_fee_group: None,
            first_slot_of_ncn_epoch: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn restaking_config(
        &mut self,
        restaking_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.restaking_config = Some(restaking_config);
        self
    }
    #[inline(always)]
    pub fn ncn_config(
        &mut self,
        ncn_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ncn_config = Some(ncn_config);
        self
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn operator(
        &mut self,
        operator: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.operator = Some(operator);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn ncn_reward_router(
        &mut self,
        ncn_reward_router: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ncn_reward_router = Some(ncn_reward_router);
        self
    }
    #[inline(always)]
    pub fn ncn_fee_group(&mut self, ncn_fee_group: u8) -> &mut Self {
        self.instruction.ncn_fee_group = Some(ncn_fee_group);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn first_slot_of_ncn_epoch(&mut self, first_slot_of_ncn_epoch: u64) -> &mut Self {
        self.instruction.first_slot_of_ncn_epoch = Some(first_slot_of_ncn_epoch);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = DistributeNcnVaultRewardsInstructionArgs {
            ncn_fee_group: self
                .instruction
                .ncn_fee_group
                .clone()
                .expect("ncn_fee_group is not set"),
            first_slot_of_ncn_epoch: self.instruction.first_slot_of_ncn_epoch.clone(),
        };
        let instruction = DistributeNcnVaultRewardsCpi {
            __program: self.instruction.__program,

            restaking_config: self
                .instruction
                .restaking_config
                .expect("restaking_config is not set"),

            ncn_config: self.instruction.ncn_config.expect("ncn_config is not set"),

            ncn: self.instruction.ncn.expect("ncn is not set"),

            operator: self.instruction.operator.expect("operator is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            ncn_reward_router: self
                .instruction
                .ncn_reward_router
                .expect("ncn_reward_router is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct DistributeNcnVaultRewardsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    restaking_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    operator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn_reward_router: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn_fee_group: Option<u8>,
    first_slot_of_ncn_epoch: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}