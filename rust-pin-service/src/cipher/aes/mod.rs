use aes_gcm::{Aes256Gcm, Nonce}; 
use aes_gcm::aead::{Aead, KeyInit};
use base64::{engine::general_purpose, Engine as _};

use crate::cipher::Algorithm;
use crate::application::env::AppEnv;

use super::Cipher;
use super::utils;

#[derive(Clone)]
pub struct Aes256Cipher{
    // key: Key<Aes256Gcm>,
    cipher: Aes256Gcm,
}

impl Aes256Cipher {
    pub fn new(env: &AppEnv) -> Self {
        if let Algorithm::Aes256Gcm = env.cipher.alg {
            let key = utils::ensure_key_len_for_aes256(env.cipher.key.clone());
            let cipher = Aes256Gcm::new(&key);
            Self { 
                // key, 
                cipher }
        } else {
            panic!("Algorithm not supported: {:?}", env.cipher.alg_str);
        }
    }
}

impl Cipher for Aes256Cipher {
    
    fn clone_box(&self) -> Box<dyn Cipher> {
        Box::new(self.clone())
    }
    
    fn encrypt(&self, pin: &[u8]) -> Vec<u8> { 
        let nonce = utils::generate_nonce();
        let mut ciphertext = self.cipher
            .encrypt(&nonce, pin)
            .expect("encryption failure!");
        let mut result = nonce.to_vec();
        result.append(&mut ciphertext);
        result
    }
    
    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        // Separate nonce and ciphertext
        let (nonce_bytes, ciphertext) = data.split_at(12); // AES-GCM expects 12-byte nonce

        let nonce = Nonce::from_slice(nonce_bytes); // 12-byte nonce
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext) // pass nonce by reference
            .expect("decryption failure!");

        plaintext
    }
    fn enc_encrypt(&self, pin: String) -> String {
        let encrypted_pin = self.encrypt(pin.as_bytes());
        general_purpose::STANDARD.encode(encrypted_pin)
    }

    fn enc_decrypt(&self, data: String) -> String {
        let decoded_data = general_purpose::STANDARD.decode(&data)
            .expect("Base64 decode failed");
        let decrypted_bytes = self.decrypt(&decoded_data);
        // Convert decrypted bytes back to String (assuming UTF-8)
        str::from_utf8(&decrypted_bytes)
            .expect("UTF-8 conversion failed")
            .to_string()
    }
}