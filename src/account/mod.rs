use crate::keysig::KeySig;

//  A droneport stores key pairs of the drones
// that belong to it
pub struct Account {
    id: Vec<u8>,
    wallets: Vec<KeySig>,
}

impl Account {
    pub fn gen_account() -> Self {
        let keypair = KeySig::new();

        let public_key = keypair.get_public_key();

        Account {
            id: public_key,
            wallets: vec![keypair],
        }
    }

    pub fn add_key_pair_to_wallet(&mut self, keysig: KeySig) {
        self.wallets.push(keysig);
    }

    pub fn sign_data(&self, data: String, i: usize) -> Vec<u8> {
        self.wallets[i]
            .sign(data.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::KeySig;
    use super::Account;

    //  Expects keysig added and id generated
    #[test]
    fn test_account_created() {
        let account = Account::gen_account();
        assert!(!String::from_utf8(account.id).unwrap()
            .is_empty());
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
}
