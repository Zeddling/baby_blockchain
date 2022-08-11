use std::borrow::BorrowMut;
use std::collections::HashMap;
use crate::account::Account;
use crate::hash::to_sha1;
use serde::Serialize;
use crate::transops::{CoinUpdates, get_nonce, Operation, Transaction};
use crate::utils::vec_to_string;

#[derive(Serialize, Clone)]
pub struct Block {
    id: String,
    previous: String,
    transactions: Vec<Transaction>
}

pub struct Blockchain {
    coin_db: HashMap<String, u8>,
    history: HashMap<String, Block>,
    transaction_db: HashMap<String, Transaction>,
    faucet_coins: u8
}

impl Block {
    pub fn create_block(transactions: Vec<Transaction >, previous: String) -> Self {
        let id = to_sha1(
            &vec_to_string(&transactions)
        );
        Block {
            id,
            previous,
            transactions
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl Blockchain {
    pub fn init() -> Self {
        let mut account = Account::gen_account();

        let operation = Operation::create_operation(
            account.clone(), account.clone(), 0);
        if !operation.verify_operation() {
            panic!("Operation is not valid");
        }
        let transaction = Transaction::create_transaction(
            vec![operation], get_nonce()
        );
        let genesis = Block::create_block(
            vec![transaction.clone()], String::from("")
        );

        let mut coin_db: HashMap<String, u8> = HashMap::new();
        coin_db.insert(
            account.get_id(),
               account.get_balance()
        );

        let mut history = HashMap::new();
        history.insert(genesis.id.clone(), genesis);

        let mut transaction_db = HashMap::new();
        transaction_db.insert(
            transaction.get_id().clone(), transaction);


        Blockchain {
            coin_db,
            history,
            transaction_db,
            faucet_coins: 100
        }
    }

    pub fn show_coin_database(&self) {
        println!(
            "{}", serde_json::to_string(
                &self.coin_db
            ).unwrap()
        )
    }

    /**
     Checks:
        1. previous exists in history
        2. block not in history
        3. block doesn't have conflicting transactions
        4. verify operations
     */
    pub fn validate_block(&mut self, mut block: Block) -> bool {
        let mut is_valid = false;

        //  1
        if self.history.contains_key(block.previous.as_str()) {
            is_valid = true;
        } else { is_valid = false; }

        //  2
        if !self.history.contains_key(
            block.id.as_str()
        ) {
            is_valid = true;
        } else { is_valid = false; }

        //  3
        for i in 0..block.transactions.len() {
            let transaction = block.transactions[i].clone();
            if !self.transaction_db.contains_key(
                transaction.get_id().as_str()
            ) {
                is_valid = true;
            } else {
                is_valid = false;
            }

            is_valid = transaction.verify_operations();
        }

        //  Add block to history and update balances
        if is_valid {
            let b = block.clone();

            //  update coin db
            for mut transaction in block.transactions.iter_mut() {
                for operation in transaction.get_operations() {
                    operation.update_coin_db(&mut self.coin_db)
                }
            }

            self.history.insert(b.id.clone(), b);

        }

        is_valid
    }

    pub fn get_history(&self) -> &HashMap<String, Block> {
        &self.history
    }

    /**
    On account create:
        1. coin db updated
        2. faucet coins and
        3. account balance after coin allocation
     */
    pub fn get_token_from_faucet(&mut self, account: &mut Account, amount: u8) {
        let old = account.get_balance();
        account.update_balance(old + amount);
        let new_balance = account.get_balance();

        self.faucet_coins -= amount;

        self.coin_db.insert(
            account.get_id(), new_balance
        );
    }

    pub fn print_blockchain(&self) {
        println!(
            "Coin database: {}",
            serde_json::to_string(&self.coin_db).unwrap()
        );
        println!(
            "History: {}",
            serde_json::to_string(&self.history).unwrap()
        );
        println!(
            "Transaction Database: {}",
            serde_json::to_string(&self.transaction_db).unwrap()
        );
        println!(
            "Faucet coins: {}",
            self.faucet_coins
        )
    }

    pub fn update_account(&self, account: &mut Account) {
        let b = self.coin_db.get(
            account.get_id().as_str()).unwrap();
        account.update_balance(
            b.clone()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::account::Account;
    use crate::blockchain::{Block, Blockchain};
    use crate::transops::{get_nonce, Operation, Transaction};

    fn get_operation() -> Operation {
        let account1 = Account::gen_account();
        let account2 = Account::gen_account();

        Operation::create_operation(account1, account2, 1)
    }

    fn get_transaction() -> Transaction {
        let op = get_operation();
        Transaction::create_transaction(
            vec![op], get_nonce()
        )
    }

    #[test]
    fn test_create_block() {
        let trans = get_transaction();
        let block = Block::create_block(
            vec![trans], "".to_string()
        );

        assert!(!block.get_id().is_empty())
    }

    #[test]
    fn test_blockchain_init() {
        let bc = Blockchain::init();
        assert!(
            !bc.get_history().is_empty()
        )
    }

    #[test]
    fn test_blockchain_add_block() {
        let mut bc = Blockchain::init();
        let mut prev = String::from("");
        let mut i = 0;
        for x in bc.history.keys() {
            prev = x.to_string();
            if i == 0 { break; }
        }

        let trans = get_transaction();


        let block = Block::create_block(
            vec![trans],
            prev
        );

        assert!(
            bc.validate_block(block)
        );
    }

    #[test]
    fn test_get_token_from_faucet() {
        let mut bc = Blockchain::init();
        let mut account = Account::gen_account();

        bc.get_token_from_faucet(&mut account, 5);

        let new_coins = bc.coin_db.get(
            account.get_id().as_str()).unwrap();
        assert_eq!(new_coins, account.get_balance().borrow())
    }
}