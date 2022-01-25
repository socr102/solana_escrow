use thiserror::Error;
use solana_program::program_error::ProgramError;
use crate::{instruction::EscrowInsutruction, error::EscrowError}

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

imple From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32);
    }
}