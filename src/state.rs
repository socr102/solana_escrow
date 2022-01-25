use solana_program::{
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
}

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64;
}

impl Sealed for Escrow {}

impl IsInitialized for Escorw {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}