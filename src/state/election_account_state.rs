use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ElectionAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub votes: HashMap<Pubkey, i64>,
    pub number_of_votes: i64,
    pub is_active: bool,
}

