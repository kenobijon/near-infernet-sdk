// lib.rs

// Import NEAR SDK features
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};

// Import other modules
mod consumer;
mod coordinator;
mod delegator;
mod manager;
mod sign_coordinator;

// Re-export structs and functions from other modules if necessary
pub use consumer::*;
pub use coordinator::*;
pub use delegator::*;
pub use manager::*;
pub use sign_coordinator::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Infernet {
    owner_id: AccountId,
    manager: manager::Manager,
    coordinator: coordinator::Coordinator,
    sign_coordinator: sign_coordinator::SignCoordinator,
}

#[near_bindgen]
impl Infernet {
    // Constructor
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: env::predecessor_account_id(),
            manager: manager::Manager::new(3600),
            coordinator: coordinator::Coordinator::new(),
            sign_coordinator: sign_coordinator::SignCoordinator::new(),
        }
    }

    // Public and private methods
    // ...

    // Methods from other modules can be included here,
    // or you can delegate to their implementations
    // in their respective modules.
}

// Include tests for this contract
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    // Function to create the VMContext
    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }
    // Set up the testing context and other necessary test setup
    // ...
    #[test]
    pub fn initialize() {
        let account_id: AccountId = "alice.near".parse().unwrap();

        // Set up the context for a new contract deployment
        let context = VMContextBuilder::new()
            .predecessor_account_id(account_id.clone())
            // You can add more parameters to the context as needed
            .build();

        testing_env!(context);

        // Instantiate the contract
        let contract = Infernet::new();
        assert_eq!(contract.owner_id, account_id);
        assert_eq!(contract.manager.get_cooldown(), 3600);
    }
}
