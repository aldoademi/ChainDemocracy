use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ResultState {
    pub results: HashMap<String, f64>
}