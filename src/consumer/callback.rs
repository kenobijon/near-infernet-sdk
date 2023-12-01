use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CallbackConsumer {
    coordinator: AccountId,
}

impl CallbackConsumer {
    pub fn new(coordinator: AccountId) -> Self {
        Self { coordinator }
    }

    pub fn request_compute(
        &self,
        container_id: String,
        inputs: Vec<u8>,
        max_gas_price: u64,
        max_gas_limit: u32,
        redundancy: u16,
    ) -> Promise {
        let period: u32 = 0;
        let frequency: u32 = 1;
        let subscription_id = env::create_subscription(
            self.coordinator.clone(),
            container_id,
            inputs,
            max_gas_price,
            max_gas_limit,
            frequency,
            period,
            redundancy,
        );
        Promise::new(subscription_id)
    }
}
