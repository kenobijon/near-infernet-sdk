use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::{ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Subscription {
    active_at: u32,
    owner: AccountId,
    max_gas_price: u128,
    redundancy: u16,
    max_gas_limit: u32,
    frequency: u32,
    period: u32,
    container_id: String,
    inputs: Vec<u8>,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Coordinator {
    id: u32,
    subscriptions: LookupMap<u32, Subscription>,
    redundancy_count: LookupMap<Vec<u8>, u16>,
}

// #[ext_contract]
// pub trait BaseConsumer {
//     fn raw_receive_compute(
//         subscription_id: u32,
//         interval: u32,
//         num_redundant_deliveries: u16,
//         sender: AccountId,
//         input: Vec<u8>,
//         output: Vec<u8>,
//         proof: Vec<u8>,
//     );
// }

#[near_bindgen]
impl Coordinator {
    #[init]
    pub fn new() -> Self {
        Self {
            id: 0,
            subscriptions: LookupMap::new(b"s".to_vec()),
            redundancy_count: LookupMap::new(b"r".to_vec()),
        }
    }

    pub fn create_subscription(
        &mut self,
        container_id: String,
        inputs: Vec<u8>,
        max_gas_price: U128,
        max_gas_limit: u32,
        frequency: u32,
        period: u32,
        redundancy: u16,
    ) -> u32 {
        let subscription_id = self.id;
        self.id += 1;

        let active_at = if period == 0 {
            env::block_timestamp() as u32
        } else {
            env::block_timestamp() as u32 + period
        };

        let owner = env::predecessor_account_id();

        let subscription = Subscription {
            active_at,
            owner,
            max_gas_price: max_gas_price.into(),
            redundancy,
            max_gas_limit,
            frequency,
            period,
            container_id,
            inputs,
        };

        self.subscriptions.insert(&subscription_id, &subscription);

        subscription_id
    }

    pub fn cancel_subscription(&mut self, subscription_id: u32) {
        let subscription = self
            .subscriptions
            .get(&subscription_id)
            .expect("Subscription not found");

        assert_eq!(
            subscription.owner,
            env::predecessor_account_id(),
            "Not subscription owner"
        );

        self.subscriptions.remove(&subscription_id);
    }

    pub fn deliver_compute(
        &mut self,
        subscription_id: u32,
        delivery_interval: u32,
        input: Vec<u8>,
        output: Vec<u8>,
        proof: Vec<u8>,
    ) {
        let subscription = self
            .subscriptions
            .get(&subscription_id)
            .expect("Subscription not found");

        assert_eq!(
            env::predecessor_account_id(),
            subscription.owner,
            "Not subscription owner"
        );

        let interval = self.get_subscription_interval(subscription.active_at, subscription.period);

        let key = self.get_key(subscription_id, interval);
        let num_redundant_deliveries = self.redundancy_count.get(&key).unwrap_or(0);

        assert!(
            num_redundant_deliveries < subscription.redundancy,
            "Interval completed"
        );

        self.redundancy_count
            .insert(&key, &(num_redundant_deliveries + 1));

        let sender = env::predecessor_account_id();

        let starting_gas = env::used_gas();

        // BaseConsumer::raw_receive_compute(
        //     subscription_id,
        //     delivery_interval,
        //     num_redundant_deliveries + 1,
        //     sender,
        //     input,
        //     output,
        //     proof,
        // );

        let ending_gas = env::prepaid_gas() - env::used_gas();

        let execution_cost = starting_gas - ending_gas;
        let overhead_cost = 0; // Add your overhead cost here
    }

    pub fn get_subscription_interval(&self, active_at: u32, period: u32) -> u32 {
        if period == 0 {
            return 1;
        }

        ((env::block_timestamp() as u64) - (active_at as u64) / 1) as u32
    }

    pub fn get_key(&self, subscription_id: u32, interval: u32) -> Vec<u8> {
        let mut key = subscription_id.to_be_bytes().to_vec();
        key.extend_from_slice(&interval.to_be_bytes());
        key
    }

    pub fn get_node_key(&self, node: String, subscription_id: u32, interval: u32) -> Vec<u8> {
        let mut key = node.as_bytes().to_vec();
        key.extend_from_slice(&subscription_id.to_be_bytes());
        key.extend_from_slice(&interval.to_be_bytes());
        key
    }
}
