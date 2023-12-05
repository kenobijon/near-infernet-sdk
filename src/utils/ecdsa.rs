use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::BorshStorageKey;
use near_sdk::PanicOnDefault;
use near_sdk::Promise;
use near_sdk::PromiseOrValue;
use near_sdk::PublicKey;
use near_sdk::StorageUsage;
use near_sdk::Gas;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CheckSignatureTestTemps {
    args_signature: Vec<u8>,
    encoded_calldata_args: Vec<u8>,
    signer: AccountId,
    expected: bool,
    success: [bool; 2],
    result: [Vec<u8>; 2],
    s: [u8; 4],
    recovered: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ECDSAContract {
    check_signature_test_temps: Vector<CheckSignatureTestTemps>,
}

#[near_bindgen]
impl ECDSAContract {
    #[init]
    pub fn new() -> Self {
        Self {
            check_signature_test_temps: Vector::new(b"check_signature_test_temps".to_vec()),
        }
    }

    /*
    Below are the functions that are used to test the checkSignature function.
    - hash:  is a fixed-length, unique representation of the data that was signed. 
    - r and s: are the two components of the signature used to sign. Prove access to the private key without revealing it
    - v: is the recovery id of the signature
    */


    pub fn recover(&self, hash: Vec<u8>, signature: Vec<u8>) -> AccountId {
        
    
    }






    // Recover the signer of a message signed by a single account.
    // @param hash The hash of the message.
    // @param signature The signature of the message.
    pub fn recover(&self, hash: Vec<u8>, signature: Vec<u8>) -> AccountId {
        // TODO: Implement the function
    }

    // Recover the signer of a message signed by a single account.
    // @param hash The hash of the message.
    // @param r The r value of the signature.
    // @param vs The vs value of the signature.
    pub fn recover_with_r_vs(&self, hash: Vec<u8>, r: Vec<u8>, vs: Vec<u8>) -> AccountId {
        // TODO: Implement the function
    }

    // Recover the signer of a message signed by a single account with a given v, r, and s.
    pub fn recover_with_v_r_s(&self, hash: Vec<u8>, v: u8, r: Vec<u8>, s: Vec<u8>) -> AccountId {
        // TODO: Implement the function
    }

    // Recover the signer of a message signed by a single account with a given hash and signature.
    pub fn recover_brutalized(&self, hash: Vec<u8>, signature: Vec<u8>) -> AccountId {
        // TODO: Implement the function
    }

    pub fn recover_brutalized_with_r_vs(&self, hash: Vec<u8>, r: Vec<u8>, vs: Vec<u8>) -> AccountId {
        // TODO: Implement the function
    }

    pub fn recover_brutalized_with_v_r_s(
        &self,
        hash: Vec<u8>,
        v: u8,
        r: Vec<u8>,
        s: Vec<u8>,
    ) -> AccountId {
        // TODO: Implement the function
    }

    pub fn test_empty_calldata_helpers(&self) {
        // TODO: Implement the function
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_example() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        let contract = ECDSAContract::new();
        contract.test_recover_with_v0_signature_with_short_eip2098_format();
        // TODO: Add more test cases
    }
}
