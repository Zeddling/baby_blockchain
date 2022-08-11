//  Handles operations and transactions

use std::collections::HashMap;
use rand::Rng;
use crate::account::Account;
use crate::hash::to_sha1;
use crate::utils::vec_to_string;
use serde::Serialize;


const flight: &str = "Cargo Flight";

#[derive(Clone, Serialize)]
pub struct Operation {
    //  Waypoint
    receiver: Account,

    //  Drone
    sender: Account,

    amount: u8,

    signature: Vec<u8>
}

#[derive(Debug)]
pub struct CoinUpdates {
    id: String,
    amount: u8
}

#[derive(Clone, Serialize)]
pub struct Transaction {
    id: String,
    operations: Vec<Operation>,
    nonce: u32
}

impl Operation {
    //  We assume that all flights are cargo related
    //  Each operation only involves 1 drone transfer
    //  from
    pub fn create_operation(
        receiver: Account, sender: Account, amount: u8
    ) -> Self {
        let signature = sender.sign_data(
            flight.to_string(), 0
        ).clone();

        Operation {
            receiver,
            sender,
            amount,
            signature
        }
    }

    pub fn get_signature(&self) -> Vec<u8> {
        self.signature.clone()
    }

    pub fn get_sender(&self) -> Account {
        self.sender.clone()
    }

    pub fn get_receiver(&self) -> Account { self.receiver.clone() }

    pub fn verify_operation(&self) -> bool {
        let mut verified = false;
        verified = self.sender.get_keysig(0).verify(
            flight.as_bytes(),
            &self.signature.clone()
        );

        if self.amount > self.sender.get_balance() {
            verified = false;
        }

        verified
    }

    pub fn update_coin_db(&self, db: &mut HashMap<String, u8>) {
        *db.get_mut(self.sender.get_id().as_str()).unwrap() -= self.amount;
        *db.get_mut(self.receiver.get_id().as_str()).unwrap() += self.amount;
    }

}

impl ToString for Operation {
    fn to_string(&self) -> String {
        let sender = self.sender.to_string();
        let receiver = self.receiver.to_string();
        let signature = hex::encode(&receiver);

        format!(
            "{}\n{}\n{}",
            sender, receiver, signature
        )
    }
}

impl Transaction {
    pub fn create_transaction(ops: Vec<Operation>, nonce: u32) -> Self {
        let id = to_sha1(
            &vec_to_string(&ops)
        );
        Transaction {
            id,
            operations: ops,
            nonce
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn verify_operations(&self) -> bool {
        let mut is_valid = false;

        for i in 0..self.operations.len() {
            if self.operations[i].verify_operation() {
                is_valid = true;
            }
        }

        is_valid
    }

    pub fn get_operations(&self) -> Vec<Operation> {
        self.operations.clone()
    }
}

impl CoinUpdates {
    pub fn new() -> Vec<CoinUpdates> {
        vec![CoinUpdates{
            id: "".to_string(), amount: 0
        }]
    }


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_amount(&self) -> u8 {
        self.amount
    }
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        let id = self.id.clone();
        let nonce = self.nonce.clone();
        let mut ops: HashMap<usize, String> = HashMap::new();
        for i in 0..self.operations.len() {
            ops.insert(
                i,
                self.operations[i].to_string()
            );
        }

        format!(
            "{}\n{}\n{}",
            id,
            serde_json::to_string(&ops).unwrap(),
            nonce
        )
    }
}

pub fn verify_operation(op: Operation) -> bool {
    let sender = op.get_sender();

    let is_verified = sender.get_keysig(0).verify(
        flight.as_bytes(), &op.get_signature()
    );

    is_verified
}

pub fn get_nonce() -> u32 {
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

        Operation::create_operation(account1, account2, 1)
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
            !trans.get_id().is_empty()
        );
    }

    #[test]
    fn test_to_string() {
        let op = get_operation();
        assert!(
            op.to_string()
                .contains("PUBLIC")
        );

        let trans = Transaction::create_transaction(
            vec![op], get_nonce()
        );
        assert!(
            trans.to_string().contains("PUBLIC")
        )
    }
}