//  Handles operations and transactions

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::Rng;
use crate::account::Account;

const flight: &str = "Cargo Flight";

pub struct Operation {
    //  Waypoint
    receiver: Account,

    //  Drone
    sender: Account,

    signature: Vec<u8>
}

pub struct Transaction {
    id: u64,
    operations: Vec<Operation>,
    nonce: u32
}

impl Operation {
    //  We assume that all flights are cargo related
    pub fn create_operation(
        receiver: Account, sender: Account
    ) -> Self {
        let signature = sender.sign_data(
            flight.to_string(), 0
        ).clone();

        Operation {
            receiver,
            sender,
            signature
        }
    }

    pub fn get_signature(&self) -> Vec<u8> {
        self.signature.clone()
    }

    pub fn get_sender(&self) -> Account {
        self.sender.clone()
    }
}

impl Transaction {
    pub fn create_transaction(ops: Vec<Operation>, nonce: u32) -> Self {
        let mut hasher = DefaultHasher::new();
        for op in 0..ops.len() {
            op.hash(&mut hasher);
        }
        let id = hasher.finish();
        println!("id: {}", id);
        Transaction {
            id,
            operations: ops,
            nonce
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}

pub fn verify_operation(op: Operation) -> bool {
    let sender = op.get_sender();

    let is_verified = sender.get_keysig(0).verify(
        flight.as_bytes(), &op.get_signature()
    );

    is_verified
}

fn get_nonce() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[cfg(test)]
mod tests {
    use crate::account::Account;
    use crate::transops::{get_nonce, Operation, Transaction, verify_operation};

    fn get_operation() -> Operation {
        let account1 = Account::gen_account();
        let account2 = Account::gen_account();

        Operation::create_operation(account1, account2)
    }

    #[test]
    fn test_create_operation() {
        let op = get_operation();
        assert!(
            !op.get_signature().is_empty()
        )
    }

    #[test]
    fn test_verify_operation() {
        let op = get_operation();

        assert!(verify_operation(op));
    }

    #[test]
    fn test_get_signature() {
        let op = get_operation();
        assert!(!op.get_signature().is_empty())
    }

    #[test]
    fn test_create_transaction() {
        let op = get_operation();
        let trans = Transaction::create_transaction(
            vec![op], get_nonce()
        );

        assert!(
            !(trans.get_id() == 0)
        );
    }
}