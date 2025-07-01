use rand::RngCore;
use aes_gcm::{Nonce, Aes256Gcm, Key}; 
use aes::cipher::consts::U12;


pub fn generate_nonce() -> Nonce<U12> {
    let mut bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut bytes);
    Nonce::from_slice(&bytes).clone()
}

pub fn ensure_key_len_for_aes256(str: String) -> Key<Aes256Gcm> {
    let mut key_bytes = str.clone().into_bytes();

    if key_bytes.len() < 32 {
        key_bytes.resize(32, b'0'); // pad with ASCII '0'
    } else if key_bytes.len() > 32 {
        key_bytes.truncate(32); 
    }
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}