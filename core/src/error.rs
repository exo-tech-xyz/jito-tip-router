use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TipRouterError {
    #[error("Zero in the denominator")]
    DenominatorIsZero = 0x2100,
    #[error("Overflow")]
    ArithmeticOverflow,
    #[error("Modulo Overflow")]
    ModuloOverflow,
    #[error("New precise number error")]
    NewPreciseNumberError,
    #[error("Cast to imprecise number error")]
    CastToImpreciseNumberError,

    #[error("Incorrect weight table admin")]
    IncorrectWeightTableAdmin = 0x2200,
    #[error("Duplicate mints in table")]
    DuplicateMintsInTable,
    #[error("There are no mints in the table")]
    NoMintsInTable,
    #[error("Too many mints for table")]
    TooManyMintsForTable,
    #[error("Weight table already initialized")]
    WeightTableAlreadyInitialized,
    #[error("Cannnot create future weight tables")]
    CannotCreateFutureWeightTables,
    #[error("Weight mints do not match - length")]
    WeightMintsDoNotMatchLength,
    #[error("Weight mints do not match - mint hash")]
    WeightMintsDoNotMatchMintHash,
    #[error("Invalid mint for weight table")]
    InvalidMintForWeightTable,
}

impl<T> DecodeError<T> for TipRouterError {
    fn type_of() -> &'static str {
        "jito::weight_table"
    }
}

impl From<TipRouterError> for ProgramError {
    fn from(e: TipRouterError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<TipRouterError> for u64 {
    fn from(e: TipRouterError) -> Self {
        e as Self
    }
}

impl From<TipRouterError> for u32 {
    fn from(e: TipRouterError) -> Self {
        e as Self
    }
}
