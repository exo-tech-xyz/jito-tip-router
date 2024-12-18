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

export const DISTRIBUTE_BASE_REWARDS_DISCRIMINATOR = 14;

export function getDistributeBaseRewardsDiscriminatorBytes() {
  return getU8Encoder().encode(DISTRIBUTE_BASE_REWARDS_DISCRIMINATOR);
}

export type DistributeBaseRewardsInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountRestakingConfig extends string | IAccountMeta<string> = string,
  TAccountNcnConfig extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountBaseRewardRouter extends string | IAccountMeta<string> = string,
  TAccountBaseFeeWallet extends string | IAccountMeta<string> = string,
  TAccountRestakingProgram extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountRestakingConfig extends string
        ? ReadonlyAccount<TAccountRestakingConfig>
        : TAccountRestakingConfig,
      TAccountNcnConfig extends string
        ? ReadonlyAccount<TAccountNcnConfig>
        : TAccountNcnConfig,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountBaseRewardRouter extends string
        ? WritableAccount<TAccountBaseRewardRouter>
        : TAccountBaseRewardRouter,
      TAccountBaseFeeWallet extends string
        ? WritableAccount<TAccountBaseFeeWallet>
        : TAccountBaseFeeWallet,
      TAccountRestakingProgram extends string
        ? ReadonlyAccount<TAccountRestakingProgram>
        : TAccountRestakingProgram,
      ...TRemainingAccounts,
    ]
  >;

export type DistributeBaseRewardsInstructionData = {
  discriminator: number;
  baseFeeGroup: number;
  epoch: bigint;
};

export type DistributeBaseRewardsInstructionDataArgs = {
  baseFeeGroup: number;
  epoch: number | bigint;
};

export function getDistributeBaseRewardsInstructionDataEncoder(): Encoder<DistributeBaseRewardsInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['baseFeeGroup', getU8Encoder()],
      ['epoch', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: DISTRIBUTE_BASE_REWARDS_DISCRIMINATOR,
    })
  );
}

export function getDistributeBaseRewardsInstructionDataDecoder(): Decoder<DistributeBaseRewardsInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['baseFeeGroup', getU8Decoder()],
    ['epoch', getU64Decoder()],
  ]);
}

export function getDistributeBaseRewardsInstructionDataCodec(): Codec<
  DistributeBaseRewardsInstructionDataArgs,
  DistributeBaseRewardsInstructionData
> {
  return combineCodec(
    getDistributeBaseRewardsInstructionDataEncoder(),
    getDistributeBaseRewardsInstructionDataDecoder()
  );
}

export type DistributeBaseRewardsInput<
  TAccountRestakingConfig extends string = string,
  TAccountNcnConfig extends string = string,
  TAccountNcn extends string = string,
  TAccountBaseRewardRouter extends string = string,
  TAccountBaseFeeWallet extends string = string,
  TAccountRestakingProgram extends string = string,
> = {
  restakingConfig: Address<TAccountRestakingConfig>;
  ncnConfig: Address<TAccountNcnConfig>;
  ncn: Address<TAccountNcn>;
  baseRewardRouter: Address<TAccountBaseRewardRouter>;
  baseFeeWallet: Address<TAccountBaseFeeWallet>;
  restakingProgram: Address<TAccountRestakingProgram>;
  baseFeeGroup: DistributeBaseRewardsInstructionDataArgs['baseFeeGroup'];
  epoch: DistributeBaseRewardsInstructionDataArgs['epoch'];
};

export function getDistributeBaseRewardsInstruction<
  TAccountRestakingConfig extends string,
  TAccountNcnConfig extends string,
  TAccountNcn extends string,
  TAccountBaseRewardRouter extends string,
  TAccountBaseFeeWallet extends string,
  TAccountRestakingProgram extends string,
  TProgramAddress extends Address = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
>(
  input: DistributeBaseRewardsInput<
    TAccountRestakingConfig,
    TAccountNcnConfig,
    TAccountNcn,
    TAccountBaseRewardRouter,
    TAccountBaseFeeWallet,
    TAccountRestakingProgram
  >,
  config?: { programAddress?: TProgramAddress }
): DistributeBaseRewardsInstruction<
  TProgramAddress,
  TAccountRestakingConfig,
  TAccountNcnConfig,
  TAccountNcn,
  TAccountBaseRewardRouter,
  TAccountBaseFeeWallet,
  TAccountRestakingProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_TIP_ROUTER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    restakingConfig: {
      value: input.restakingConfig ?? null,
      isWritable: false,
    },
    ncnConfig: { value: input.ncnConfig ?? null, isWritable: false },
    ncn: { value: input.ncn ?? null, isWritable: false },
    baseRewardRouter: {
      value: input.baseRewardRouter ?? null,
      isWritable: true,
    },
    baseFeeWallet: { value: input.baseFeeWallet ?? null, isWritable: true },
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
      getAccountMeta(accounts.restakingConfig),
      getAccountMeta(accounts.ncnConfig),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.baseRewardRouter),
      getAccountMeta(accounts.baseFeeWallet),
      getAccountMeta(accounts.restakingProgram),
    ],
    programAddress,
    data: getDistributeBaseRewardsInstructionDataEncoder().encode(
      args as DistributeBaseRewardsInstructionDataArgs
    ),
  } as DistributeBaseRewardsInstruction<
    TProgramAddress,
    TAccountRestakingConfig,
    TAccountNcnConfig,
    TAccountNcn,
    TAccountBaseRewardRouter,
    TAccountBaseFeeWallet,
    TAccountRestakingProgram
  >;

  return instruction;
}

export type ParsedDistributeBaseRewardsInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    restakingConfig: TAccountMetas[0];
    ncnConfig: TAccountMetas[1];
    ncn: TAccountMetas[2];
    baseRewardRouter: TAccountMetas[3];
    baseFeeWallet: TAccountMetas[4];
    restakingProgram: TAccountMetas[5];
  };
  data: DistributeBaseRewardsInstructionData;
};

export function parseDistributeBaseRewardsInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDistributeBaseRewardsInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 6) {
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
      restakingConfig: getNextAccount(),
      ncnConfig: getNextAccount(),
      ncn: getNextAccount(),
      baseRewardRouter: getNextAccount(),
      baseFeeWallet: getNextAccount(),
      restakingProgram: getNextAccount(),
    },
    data: getDistributeBaseRewardsInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
