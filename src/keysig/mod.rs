use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    sign::{Signer, Verifier},
};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;

extern crate openssl;

#[derive(Debug, Clone)]
pub struct KeySig {
    keypair: Rsa<Private>,
}

impl KeySig {
    //  Get public key
    pub fn get_public_key(&self) -> Vec<u8> {
        self.keypair.clone()
            .public_key_to_pem().unwrap()
    }
    //  Generate key pairs and signature
    pub fn new() -> Self {
        //  Generate keypair
        let keypair = Rsa::generate(2048).unwrap();

        KeySig { keypair: keypair }
    }

    //  Sign data
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let keypair = PKey::from_rsa(self.keypair.clone()).unwrap();
        let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        signer.update(data).unwrap();
        signer.sign_to_vec().unwrap()
    }

    pub fn to_string(&self) -> String {
        let public_key = self.keypair.public_key_to_pem().unwrap();
        let private_key = self.keypair.private_key_to_pem().unwrap();

        format!(
            "{}\n{}",
            String::from_utf8(private_key.clone()).unwrap(),
            String::from_utf8(public_key.clone()).unwrap()
        )
    }

    pub fn verify(&self, data: &[u8], signature: &Vec<u8>) -> bool {
        let keypair = PKey::from_rsa(self.keypair.clone()).unwrap();
        let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
        verifier.update(data).unwrap();
        verifier.verify(signature).unwrap()
    }
}

impl ToString for KeySig {
    fn to_string(&self) -> String {
        let public_key = self.keypair.public_key_to_pem().unwrap();
        let private_key = self.keypair.private_key_to_pem().unwrap();

        format!(
            "{}\n{}",
            hex::encode(private_key),
            hex::encode(public_key)
        )
    }
}

impl Serialize for KeySig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let pub_key = hex::encode(self.get_public_key());
        let priv_key = hex::encode(
            self.keypair.private_key_to_pem().unwrap()
        );

        let mut state = serializer.serialize_struct("KeySig", 2)?;
        state.serialize_field("public key", &pub_key)?;
        state.serialize_field("private key", &priv_key)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};
    use super::KeySig;

    #[test]
    fn test_to_string() {
        let keysig = KeySig::new();

        assert!(keysig
            .to_string()
            .contains("-----BEGIN RSA PRIVATE KEY-----"))
    }

    #[test]
    fn test_sign_and_verify() {
        let keysig = KeySig::new();

        let data = b"Hello World";
        let signature = keysig.sign(data);

        assert!(keysig.verify(data, &signature));
    }

    #[test]
    fn test_get_public_key() {
        let keysig = KeySig::new();
        let pub_key = keysig.get_public_key();
        let str_pub_key = String::from_utf8(
            pub_key).unwrap();

        assert!(!str_pub_key.is_empty())
    }

}
