use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
pub fn generate_keys() -> (RsaPrivateKey,RsaPublicKey) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    (priv_key, pub_key)
}

pub fn decrypt_str(priv_key: &RsaPrivateKey, ciphertext: &str) -> Vec<u8> {
    let ciphertext = ciphertext.as_bytes();
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &ciphertext).expect("failed to decrypt");
    return dec_data;
}

pub fn encrypt_str(pub_key: &RsaPublicKey, message: &str) -> Vec<u8> {
    // 将字符串转换为字节数组
    let message_bytes = message.as_bytes();
    let mut rng2 = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng2, Pkcs1v15Encrypt, message_bytes).expect("failed to encrypt");
    return enc_data;
}

pub fn decrypt_u8(priv_key: &RsaPrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &ciphertext).expect("failed to decrypt");
    return dec_data;
}

pub fn encrypt_u8(pub_key: &RsaPublicKey, message: &[u8]) -> Vec<u8> {


    let mut rng2 = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng2, Pkcs1v15Encrypt, message).expect("failed to encrypt");
    return enc_data;
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_encrypt_decrypt() {
        let (priv_key, pub_key) = generate_keys();
        let message = "Hello, world!";
        let encrypted = encrypt_str(&pub_key, message);
        let decrypted = decrypt_u8(&priv_key,&encrypted);
    }
    
    #[test]
    fn test_encrypt_decrypt_u8() {
        let (priv_key, pub_key) = generate_keys();
        let message = "Hello, world!";
        let encrypted = encrypt_u8(&pub_key, message.as_bytes());
        let decrypted = decrypt_u8(&priv_key,&encrypted);
    }
} 