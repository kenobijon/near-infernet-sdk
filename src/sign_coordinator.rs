use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env;
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json, to_vec};
use near_sdk::PublicKey;
use near_sdk::{ext_contract, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

use crate::coordinator;
use crate::delegator;

use coordinator::Coordinator;
use delegator::Delegator;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Subscription {
    owner: AccountId,
    active_at: u32,
    period: u32,
    frequency: u32,
    redundancy: u16,
    max_gas_price: U64,
    max_gas_limit: u32,
    container_id: String,
    inputs: Base64VecU8,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct SignCoordinator {
    max_subscriber_nonce: LookupMap<AccountId, u32>,
    delegate_created_ids: LookupMap<Vec<u8>, u32>,
    subscriptions: LookupMap<u32, Subscription>,
}

// impl Delegator for SignCoordinator {
//     fn signer(&self) -> AccountId {
//         env::predecessor_account_id()
//     }
// }

#[near_bindgen]
impl SignCoordinator {
    #[init]
    pub fn new() -> Self {
        Self {
            max_subscriber_nonce: LookupMap::new(b"m".to_vec()),
            delegate_created_ids: LookupMap::new(b"d".to_vec()),
            subscriptions: LookupMap::new(b"s".to_vec()),
        }
    }

    #[payable]
    pub fn create_subscription_delegate(
        &mut self,
        nonce: u32,
        expiry: u32,
        sub: Subscription,
        v: u8,
        r: Vec<u8>,
        s: Vec<u8>,
    ) -> PromiseOrValue<(u32, bool)> {
        // Check if subscription already exists via delegate-created lookup table
        let key = env::sha256(&to_vec(&(sub.owner.clone(), nonce)).unwrap());
        if let Some(subscription_id) = self.delegate_created_ids.get(&key) {
            return PromiseOrValue::Value((subscription_id, true));
        }

        // Else, if subscription does not exist
        // First, verify that signature has not expired
        if env::block_timestamp() >= expiry.into() {
            env::panic_str("SignatureExpired");
        }

        // Generate EIP-712 data
        let digest = env::sha256(
            &to_vec(&json!({
                "nonce": nonce,
                "expiry": expiry,
                "sub": sub,
            }))
            .unwrap(),
        );

        // Get recovered signer from data
        let recovered_signer = env::ecrecover(&digest, &s, v, false).unwrap();
        let key_slice = &recovered_signer[..];
        let recovered_pk = PublicKey::try_from_slice(key_slice).expect("Invalid public key format");

        let delegated_signer = env::signer_account_pk();

        // pub fn ecrecover(hash: &[u8], signature: &[u8], v: u8, malleability_flag: bool) -> Option<[u8; 64]>

        // let delegated_signer = Delegator::get_signer(&self);

        // Verify signatures (recovered_signer should equal delegated_signer)
        if recovered_pk != delegated_signer {
            env::panic_str("SignerMismatch");
        }

        // By this point, the signer is verified and a net-new subscription can be created
        // Assign new subscription id
        // let subscription_id = self.subscriptions.len() as u32;
        let subscription_id = 0;

        // Store provided subscription as-is
        self.subscriptions.insert(&subscription_id, &sub);

        // Update delegate-created ID lookup table
        self.delegate_created_ids.insert(&key, &subscription_id);

        // Emit new subscription
        env::log(format!("SubscriptionCreated: {}", subscription_id).as_bytes());

        // Update max known subscriber nonce (useful for off-chain signing utilities to prevent nonce-collision)
        if nonce > self.max_subscriber_nonce.get(&sub.owner).unwrap_or(0) {
            self.max_subscriber_nonce.insert(&sub.owner, &nonce);
        }

        // Explicitly return subscriptionId
        PromiseOrValue::Value((subscription_id, false))
    }

    pub fn deliver_compute_delegatee(
        &mut self,
        nonce: u32,
        expiry: u32,
        sub: Subscription,
        v: u8,
        r: Vec<u8>,
        s: Vec<u8>,
        delivery_interval: u32,
        input: Base64VecU8,
        output: Base64VecU8,
        proof: Base64VecU8,
    ) {
        // Create subscriptionId via delegatee creation + or collect if subscription already exists
        // let near_sdk::PromiseOrValue::Value(value): PromiseOrValue<(u32, bool)> = self
        //     .create_subscription_delegate(nonce, expiry, sub, v, r, s)
        //     .try_into()
        //     .unwrap();

        // unpack tuple
        // let (subscription_id, cached) = value;
        // Calculate additional gas overhead imposed from delivering container compute response via delegatee function
        // let overhead = if cached {
        //     // Subscription exists, cost to retrieve subscriptionId
        //     600
        // } else {
        //     // Subscription does not exist, cost to create subscription w/ delegatee signature
        //     91_200
        // };

        // Deliver subscription response
        // self.deliver_compute_with_overhead(
        //     subscription_id,
        //     delivery_interval,
        //     input,
        //     output,
        //     proof,
        //     overhead,
        // );
    }
}
