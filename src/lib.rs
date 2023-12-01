// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::near_bindgen;

// Import the contract macros
mod coordinator;
mod delegator;
mod manager;
mod sign_coordinator;

pub use crate::coordinator::Coordinator;
pub use crate::manager::Manager;
pub use crate::sign_coordinator::SignCoordinator;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    manager: Manager,
    coordinator: Coordinator,
    sign_coordinator: SignCoordinator,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            manager: Manager::default(),
            coordinator: Coordinator::default(),
            sign_coordinator: SignCoordinator::default(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self::default()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    // Import test utilities from NEAR SDK
    #[test]
    fn test_deploy() {
        // Use the helper functions in the `test_utils` module of the NEAR SDK
        // to set the contract context.
        // See https://docs.rs/near-sdk/0.4.0/near_sdk/testing/index.html
        // for more info.
        near_sdk::test_utils::test_env::setup();
        // Use the macro to instantiate the contract.
        let contract = Contract::default();
        // Verify the state in the contract's storage
        assert!(contract.manager);
        assert!(contract.coordinator);
        assert!(contract.sign_coordinator)
    }
}
