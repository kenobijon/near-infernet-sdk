use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Delegator {
    signer: String,
}

#[near_bindgen]
impl Delegator {
    #[init]
    pub fn new(signer: String) -> Self {
        Self { signer }
    }

    #[private]
    pub fn update_signer(&mut self, new_signer: String) {
        self.signer = new_signer;
    }

    pub fn get_signer(&self) -> String {
        self.signer.clone()
    }
}
