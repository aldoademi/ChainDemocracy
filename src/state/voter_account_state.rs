
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VoterAccountState {
   pub election_card_number: String
}

