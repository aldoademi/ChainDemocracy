
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VoterAccountState {
   pub election_card_number: String,
   pub voted: Pubkey
}

