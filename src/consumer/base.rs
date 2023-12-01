use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct BaseConsumer {
    coordinator: String,
}

#[near_bindgen]
impl BaseConsumer {
    pub fn new(coordinator: String) -> Self {
        Self { coordinator }
    }

    pub fn raw_receive_compute(
        &self,
        subscription_id: u32,
        interval: u32,
        redundancy: u16,
        node: String,
        input: Vec<u8>,
        output: Vec<u8>,
        proof: Vec<u8>,
    ) {
        // Ensure caller is coordinator
        assert_eq!(
            env::predecessor_account_id(),
            self.coordinator,
            "NotCoordinator"
        );

        // Call internal receive function, since caller is validated
        self._receive_compute(
            subscription_id,
            interval,
            redundancy,
            node,
            input,
            output,
            proof,
        );
    }

    #[private]
    fn _receive_compute(
        &self,
        subscription_id: u32,
        interval: u32,
        redundancy: u16,
        node: String,
        input: Vec<u8>,
        output: Vec<u8>,
        proof: Vec<u8>,
    ) {
        // Handle compute response logic here
    }
}
