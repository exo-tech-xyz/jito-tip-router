/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type WritableAccount,
} from '@solana/web3.js';
import { JITO_TIP_ROUTER_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const SNAPSHOT_VAULT_OPERATOR_DELEGATION_DISCRIMINATOR = 10;

export function getSnapshotVaultOperatorDelegationDiscriminatorBytes() {
  return getU8Encoder().encode(
    SNAPSHOT_VAULT_OPERATOR_DELEGATION_DISCRIMINATOR
  );
}

export type SnapshotVaultOperatorDelegationInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountRestakingConfig extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountOperator extends string | IAccountMeta<string> = string,
  TAccountVault extends string | IAccountMeta<string> = string,
  TAccountVaultNcnTicket extends string | IAccountMeta<string> = string,
  TAccountNcnVaultTicket extends string | IAccountMeta<string> = string,
  TAccountVaultOperatorDelegation extends
    | string
    | IAccountMeta<string> = string,
  TAccountWeightTable extends string | IAccountMeta<string> = string,
  TAccountEpochSnapshot extends string | IAccountMeta<string> = string,
  TAccountOperatorSnapshot extends string | IAccountMeta<string> = string,
  TAccountVaultProgram extends string | IAccountMeta<string> = string,
  TAccountRestakingProgram extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountRestakingConfig extends string
        ? ReadonlyAccount<TAccountRestakingConfig>
        : TAccountRestakingConfig,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountOperator extends string
        ? ReadonlyAccount<TAccountOperator>
        : TAccountOperator,
      TAccountVault extends string
        ? ReadonlyAccount<TAccountVault>
        : TAccountVault,
      TAccountVaultNcnTicket extends string
        ? ReadonlyAccount<TAccountVaultNcnTicket>
        : TAccountVaultNcnTicket,
      TAccountNcnVaultTicket extends string
        ? ReadonlyAccount<TAccountNcnVaultTicket>
        : TAccountNcnVaultTicket,
      TAccountVaultOperatorDelegation extends string
        ? ReadonlyAccount<TAccountVaultOperatorDelegation>
        : TAccountVaultOperatorDelegation,
      TAccountWeightTable extends string
        ? ReadonlyAccount<TAccountWeightTable>
        : TAccountWeightTable,
      TAccountEpochSnapshot extends string
        ? WritableAccount<TAccountEpochSnapshot>
        : TAccountEpochSnapshot,
      TAccountOperatorSnapshot extends string
        ? WritableAccount<TAccountOperatorSnapshot>
        : TAccountOperatorSnapshot,
      TAccountVaultProgram extends string
        ? ReadonlyAccount<TAccountVaultProgram>
        : TAccountVaultProgram,
      TAccountRestakingProgram extends string
        ? ReadonlyAccount<TAccountRestakingProgram>
        : TAccountRestakingProgram,
      ...TRemainingAccounts,
    ]
  >;

export type SnapshotVaultOperatorDelegationInstructionData = {
  discriminator: number;
  epoch: bigint;
};

export type SnapshotVaultOperatorDelegationInstructionDataArgs = {
  epoch: number | bigint;
};

export function getSnapshotVaultOperatorDelegationInstructionDataEncoder(): Encoder<SnapshotVaultOperatorDelegationInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['epoch', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: SNAPSHOT_VAULT_OPERATOR_DELEGATION_DISCRIMINATOR,
    })
  );
}

export function getSnapshotVaultOperatorDelegationInstructionDataDecoder(): Decoder<SnapshotVaultOperatorDelegationInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['epoch', getU64Decoder()],
  ]);
}

export function getSnapshotVaultOperatorDelegationInstructionDataCodec(): Codec<
  SnapshotVaultOperatorDelegationInstructionDataArgs,
  SnapshotVaultOperatorDelegationInstructionData
> {
  return combineCodec(
    getSnapshotVaultOperatorDelegationInstructionDataEncoder(),
    getSnapshotVaultOperatorDelegationInstructionDataDecoder()
  );
}

export type SnapshotVaultOperatorDelegationInput<
  TAccountConfig extends string = string,
  TAccountRestakingConfig extends string = string,
  TAccountNcn extends string = string,
  TAccountOperator extends string = string,
  TAccountVault extends string = string,
  TAccountVaultNcnTicket extends string = string,
  TAccountNcnVaultTicket extends string = string,
  TAccountVaultOperatorDelegation extends string = string,
  TAccountWeightTable extends string = string,
  TAccountEpochSnapshot extends string = string,
  TAccountOperatorSnapshot extends string = string,
  TAccountVaultProgram extends string = string,
  TAccountRestakingProgram extends string = string,
> = {
  config: Address<TAccountConfig>;
  restakingConfig: Address<TAccountRestakingConfig>;
  ncn: Address<TAccountNcn>;
  operator: Address<TAccountOperator>;
  vault: Address<TAccountVault>;
  vaultNcnTicket: Address<TAccountVaultNcnTicket>;
  ncnVaultTicket: Address<TAccountNcnVaultTicket>;
  vaultOperatorDelegation: Address<TAccountVaultOperatorDelegation>;
  weightTable: Address<TAccountWeightTable>;
  epochSnapshot: Address<TAccountEpochSnapshot>;
  operatorSnapshot: Address<TAccountOperatorSnapshot>;
  vaultProgram: Address<TAccountVaultProgram>;
  restakingProgram: Address<TAccountRestakingProgram>;
  epoch: SnapshotVaultOperatorDelegationInstructionDataArgs['epoch'];
};

export function getSnapshotVaultOperatorDelegationInstruction<
  TAccountConfig extends string,
  TAccountRestakingConfig extends string,
  TAccountNcn extends string,
  TAccountOperator extends string,
  TAccountVault extends string,
  TAccountVaultNcnTicket extends string,
  TAccountNcnVaultTicket extends string,
  TAccountVaultOperatorDelegation extends string,
  TAccountWeightTable extends string,
  TAccountEpochSnapshot extends string,
  TAccountOperatorSnapshot extends string,
  TAccountVaultProgram extends string,
  TAccountRestakingProgram extends string,
  TProgramAddress extends Address = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
>(
  input: SnapshotVaultOperatorDelegationInput<
    TAccountConfig,
    TAccountRestakingConfig,
    TAccountNcn,
    TAccountOperator,
    TAccountVault,
    TAccountVaultNcnTicket,
    TAccountNcnVaultTicket,
    TAccountVaultOperatorDelegation,
    TAccountWeightTable,
    TAccountEpochSnapshot,
    TAccountOperatorSnapshot,
    TAccountVaultProgram,
    TAccountRestakingProgram
  >,
  config?: { programAddress?: TProgramAddress }
): SnapshotVaultOperatorDelegationInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountRestakingConfig,
  TAccountNcn,
  TAccountOperator,
  TAccountVault,
  TAccountVaultNcnTicket,
  TAccountNcnVaultTicket,
  TAccountVaultOperatorDelegation,
  TAccountWeightTable,
  TAccountEpochSnapshot,
  TAccountOperatorSnapshot,
  TAccountVaultProgram,
  TAccountRestakingProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_TIP_ROUTER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    restakingConfig: {
      value: input.restakingConfig ?? null,
      isWritable: false,
    },
    ncn: { value: input.ncn ?? null, isWritable: false },
    operator: { value: input.operator ?? null, isWritable: false },
    vault: { value: input.vault ?? null, isWritable: false },
    vaultNcnTicket: { value: input.vaultNcnTicket ?? null, isWritable: false },
    ncnVaultTicket: { value: input.ncnVaultTicket ?? null, isWritable: false },
    vaultOperatorDelegation: {
      value: input.vaultOperatorDelegation ?? null,
      isWritable: false,
    },
    weightTable: { value: input.weightTable ?? null, isWritable: false },
    epochSnapshot: { value: input.epochSnapshot ?? null, isWritable: true },
    operatorSnapshot: {
      value: input.operatorSnapshot ?? null,
      isWritable: true,
    },
    vaultProgram: { value: input.vaultProgram ?? null, isWritable: false },
    restakingProgram: {
      value: input.restakingProgram ?? null,
      isWritable: false,
    },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.restakingConfig),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.operator),
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.vaultNcnTicket),
      getAccountMeta(accounts.ncnVaultTicket),
      getAccountMeta(accounts.vaultOperatorDelegation),
      getAccountMeta(accounts.weightTable),
      getAccountMeta(accounts.epochSnapshot),
      getAccountMeta(accounts.operatorSnapshot),
      getAccountMeta(accounts.vaultProgram),
      getAccountMeta(accounts.restakingProgram),
    ],
    programAddress,
    data: getSnapshotVaultOperatorDelegationInstructionDataEncoder().encode(
      args as SnapshotVaultOperatorDelegationInstructionDataArgs
    ),
  } as SnapshotVaultOperatorDelegationInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountRestakingConfig,
    TAccountNcn,
    TAccountOperator,
    TAccountVault,
    TAccountVaultNcnTicket,
    TAccountNcnVaultTicket,
    TAccountVaultOperatorDelegation,
    TAccountWeightTable,
    TAccountEpochSnapshot,
    TAccountOperatorSnapshot,
    TAccountVaultProgram,
    TAccountRestakingProgram
  >;

  return instruction;
}

export type ParsedSnapshotVaultOperatorDelegationInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    restakingConfig: TAccountMetas[1];
    ncn: TAccountMetas[2];
    operator: TAccountMetas[3];
    vault: TAccountMetas[4];
    vaultNcnTicket: TAccountMetas[5];
    ncnVaultTicket: TAccountMetas[6];
    vaultOperatorDelegation: TAccountMetas[7];
    weightTable: TAccountMetas[8];
    epochSnapshot: TAccountMetas[9];
    operatorSnapshot: TAccountMetas[10];
    vaultProgram: TAccountMetas[11];
    restakingProgram: TAccountMetas[12];
  };
  data: SnapshotVaultOperatorDelegationInstructionData;
};

export function parseSnapshotVaultOperatorDelegationInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedSnapshotVaultOperatorDelegationInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 13) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      restakingConfig: getNextAccount(),
      ncn: getNextAccount(),
      operator: getNextAccount(),
      vault: getNextAccount(),
      vaultNcnTicket: getNextAccount(),
      ncnVaultTicket: getNextAccount(),
      vaultOperatorDelegation: getNextAccount(),
      weightTable: getNextAccount(),
      epochSnapshot: getNextAccount(),
      operatorSnapshot: getNextAccount(),
      vaultProgram: getNextAccount(),
      restakingProgram: getNextAccount(),
    },
    data: getSnapshotVaultOperatorDelegationInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
