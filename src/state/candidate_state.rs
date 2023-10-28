use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CandidateState {
    pub is_initialized: bool,
    pub first_name: String,
    pub last_name: String,
}