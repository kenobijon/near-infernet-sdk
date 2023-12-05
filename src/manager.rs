// SPDX-License-Identifier: BSD-3-Clause-Clear
// Rust Smart Contract for NEAR Protocol

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum NodeStatus {
    Inactive,
    Registered,
    Active,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct NodeInfo {
    pub status: NodeStatus,
    pub cooldown_start: u32,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Manager {
    cooldown: u32,
    node_info: LookupMap<AccountId, NodeInfo>,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            cooldown: 3600, // 1 hour in seconds
            node_info: LookupMap::new(b"n".to_vec()),
        }
    }
}

#[near_bindgen]
impl Manager {
    #[init]
    pub fn new(cooldown: u32) -> Self {
        Self {
            cooldown,
            node_info: LookupMap::new(b"n".to_vec()),
        }
    }

    pub fn register_node(&mut self, node: AccountId) {
        let account_id = env::predecessor_account_id();
        let mut info = self.node_info.get(&node).unwrap_or_else(|| NodeInfo {
            status: NodeStatus::Inactive,
            cooldown_start: 0,
        });

        if info.status != NodeStatus::Inactive {
            env::panic_str("NodeNotRegisterable");
        }

        info.status = NodeStatus::Registered;
        info.cooldown_start = env::block_timestamp() as u32;

        self.node_info.insert(&node, &info);
        env::log(format!("NodeRegistered: {}", node).as_bytes());
    }

    pub fn activate_node(&mut self) {
        let account_id = env::predecessor_account_id();
        let mut info = self.node_info.get(&account_id).unwrap_or_else(|| {
            env::panic_str("NodeNotActivateable");
        });

        if info.status != NodeStatus::Registered {
            env::panic_str("NodeNotActivateable");
        }

        let cooldown_end = info.cooldown_start + self.cooldown;
        if (env::block_timestamp() as u32) < cooldown_end {
            env::panic_str("CooldownActive");
        }

        info.status = NodeStatus::Active;
        info.cooldown_start = 0;

        self.node_info.insert(&account_id, &info);
        env::log(format!("NodeActivated: {}", account_id).as_bytes());
    }

    pub fn deactivate_node(&mut self) {
        let account_id = env::predecessor_account_id();
        self.node_info.remove(&account_id);
        env::log(format!("NodeDeactivated: {}", account_id).as_bytes());
    }

    pub fn get_cooldown(&self) -> u32 {
        self.cooldown
    }
}
