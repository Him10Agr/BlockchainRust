use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Sign};
use sha2::{Digest, Sha256};

pub fn generate_rsa_key_pair() -> (RsaPrivateKey, RsaPublicKey) {

    let bit_size = 2048;
    let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), bit_size).
                        expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    return (private_key, public_key);
}

pub fn sign_message(private_key: &RsaPrivateKey, message: &str) -> Vec<u8> {

    let mut hasher = Sha256::new();
    hasher.update(message);
    let hash = hasher.finalize();
    return private_key.sign(Pkcs1v15Sign::new::<Sha256>(), &hash).expect("Failed to sign message");
}

pub fn verify_signature(public_key: &RsaPublicKey, message: &str, signature: &[u8]) -> bool {

    let mut hasher = Sha256::new();
    hasher.update(message);
    let hash = hasher.finalize();
    return public_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hash, signature).is_ok();
}