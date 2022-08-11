use crate::account::Account;
use crate::blockchain::{Block, Blockchain};
use crate::transops::{get_nonce, Operation, Transaction};

mod account;
mod blockchain;
mod hash;
mod keysig;
mod transops;
mod utils;

fn main() {
    let mut bc = Blockchain::init();
    //  Genesis block to create
    let mut gen_block = String::from("");
    let i = 0;
    for x in bc.get_history().keys() {
        gen_block = x.to_string();
        if i == 0 { break; }
    }
    bc.print_blockchain();

    //  Create account
    let mut account1 = Account::gen_account();
    let mut account2 = Account::gen_account();
    bc.get_token_from_faucet(&mut account1, 10);
    bc.get_token_from_faucet(&mut account2, 10);
    print_separator();
    bc.print_blockchain();

    // Simulate sending drones from account1 to account2
    let operation = Operation::create_operation(
        account2.clone(), account1.clone(), 4
    );
    let op2 = Operation::create_operation(
        account1.clone(), account2.clone(), 6
    );
    let transaction = Transaction::create_transaction(
        vec![operation, op2], get_nonce());

    let block = Block::create_block(
        vec![transaction], gen_block
    );
    bc.validate_block(block);

    //  If it's valid it should be added to the chain
    bc.update_account(&mut account1);
    bc.update_account(&mut account2);
    bc.print_blockchain();

    println!("Account 1 balance: {}", account1.get_balance());
    println!("Account 2 balance: {}", account2.get_balance());
}

fn print_separator() {
    println!("{:=<300}", "");
}

