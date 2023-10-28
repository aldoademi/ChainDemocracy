use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ResultState {
    pub results: HashMap<String, f32>,
    pub number_of_votes: i64
}