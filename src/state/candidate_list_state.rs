use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize,BorshDeserialize)]
pub struct CandidateListState{
    pub is_initialized: bool,
    pub candidate: HashMap<String, Pubkey>
}
