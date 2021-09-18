use ecies::utils::generate_keypair;

pub fn test_generate_keypair() {
    let (sk1, pk1) = generate_keypair();
    let (sk2, pk2) = generate_keypair();
    assert_ne!(sk1, sk2);
    assert_ne!(pk1, pk2);
}

use ecies::pure_aes::{aead_encrypt, aead_decrypt};
use ring::aead;

pub fn test_aead_enc_then_dec() {
    let plain_text: [u8; 5] = [0xde, 0xff, 0xab, 0xcd, 0x90];
    let key = [0x90u8; 32];
    let iv = [0x89u8; 12];

    let mut buf = plain_text.to_vec();
    aead_encrypt(&aead::AES_256_GCM, &mut buf, &key, &iv).unwrap();
    let result = aead_decrypt(&aead::AES_256_GCM, &mut buf, &key, &iv).unwrap();
    assert_eq!(&result[..], &plain_text[..]);
}