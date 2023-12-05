use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

use crate::Coordinator;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CallbackConsumer {
    coordinator: Coordinator,
}

impl CallbackConsumer {
    pub fn new(coordinator: AccountId) -> Self {
        Self {
            coordinator: Coordinator::new(),
        }
    }

    pub fn request_compute(
        &mut self,
        container_id: String,
        inputs: Vec<u8>,
        max_gas_price: U128,
        max_gas_limit: u32,
        redundancy: u16,
    ) -> u32 {
        let period: u32 = 0;
        let frequency: u32 = 1;
        let subscription_id = self.coordinator.create_subscription(
            container_id,
            inputs,
            max_gas_price,
            max_gas_limit,
            frequency,
            period,
            redundancy,
        );

        subscription_id
    }
}
