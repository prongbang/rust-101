
// module using module
use crate::cryptography::Cryptography;

pub struct Aes {
    key: String,
}

impl Aes {
    pub fn new(key: String) -> Aes {
        Aes { key }
    }
}

// Implement Cryptography trait to Aes struct
impl Cryptography for Aes {
    fn encrypt(&self, plaintext: &str) -> String {
        format!("key = {}, plaintext = {}", self.key, plaintext)
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        format!("key = {}, ciphertext = {}", self.key, ciphertext)
    }
}