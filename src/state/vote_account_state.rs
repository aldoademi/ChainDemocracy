use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VoteAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub votes: HashMap<Pubkey, Vec<Pubkey>>,
    pub is_active: bool,
}

