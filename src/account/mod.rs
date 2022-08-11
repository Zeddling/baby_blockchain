use std::collections::HashMap;
use serde::Serialize;

use crate::keysig::KeySig;

/**
    A droneport stores key pairs of the drones
    that belong to it.
    A drone represents a coin
 */
#[derive(Clone, Serialize)]
pub struct Account {
    id: String,
    wallets: Vec<KeySig>,
    balance: u8
}

impl Account {
    pub fn gen_account() -> Self {
        let keypair = KeySig::new();

        let public_key = hex::encode(
            keypair.get_public_key()
        );

        Account {
            id: public_key,
            wallets: vec![keypair],
            balance: 0
        }
    }

    pub fn add_key_pair_to_wallet(&mut self, keysig: KeySig) {
        self.wallets.push(keysig);
    }

    pub fn sign_data(&self, data: String, i: usize) -> Vec<u8> {
        self.wallets[i]
            .sign(data.as_bytes())
    }
    
    pub fn get_keysig(&self, i: usize) -> KeySig {
        self.wallets[i].clone()
    }

    pub fn get_balance(&self) -> u8 {
        self.balance
    }

    pub fn update_balance(&mut self, balance: u8) {
        self.balance = balance;
    }

    pub fn print_balance(&self) {
        println!("Balance: {}", self.balance);
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl ToString for Account {
    fn to_string(&self) -> String {
        let id = hex::encode(self.id.clone());

        let mut wallet: HashMap<usize, String> = HashMap::new();
        for i in 0..self.wallets.len() {
            wallet.insert(
                i,
                self.wallets[i].to_string()
            );
        }
        format!(
            "{}\n{}\n{}",
            id,
            serde_json::to_string(&wallet).unwrap(),
            self.balance
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::keysig::KeySig;
    use super::Account;

    //  Expects keysig added and id generated
    #[test]
    fn test_account_created() {
        let account = Account::gen_account();
        assert!(account.wallets[0]
            .to_string()
            .contains("-----BEGIN RSA PRIVATE KEY-----"))
    }

    #[test]
    fn test_add_key_pair_to_wallet() {
        let mut account = Account::gen_account();
        let new_key_sig = KeySig::new();
        account.add_key_pair_to_wallet(new_key_sig.clone());

        assert!(
            account.wallets[1].to_string().eq(
                &new_key_sig.to_string()
            )
        )
    }

    #[test]
    fn test_sign_data() {
        let account = Account::gen_account();
        let signature = account.sign_data(
            String::from("Hello World"), 0);

        assert!(
            account.wallets[0]
                .verify(b"Hello World", &signature)
        );
    }

    #[test]
    fn test_update_balance() {
        let mut account = Account::gen_account();
        let prev = account.get_balance();
        let new_bal = prev + 1;
        account.update_balance(new_bal);

        assert!(
            prev < account.get_balance()
        );
    }

    #[test]
    fn test_get_balance() {
        let account = Account::gen_account();
        let b = account.get_balance();
        assert_eq!(b, 1);
    }

}
