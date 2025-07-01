mod utils;
pub mod aes;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum Algorithm{
    #[serde(rename = "aes128")]
    Aes128Gcm,
    #[serde(rename = "aes256")]
    Aes256Gcm
}

pub trait Cipher {
    fn clone_box(&self) -> Box<dyn Cipher>;
    fn encrypt(&self, pin: &[u8]) -> Vec<u8>;
    fn decrypt(&self, data: &[u8]) -> Vec<u8>;
    fn enc_encrypt(&self, data: String) -> String;
    fn enc_decrypt(&self, data: String) -> String;
}

impl Clone for Box<dyn Cipher> {
    fn clone(&self) -> Box<dyn Cipher> {
        self.clone_box()
    }
}