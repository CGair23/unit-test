
#![no_std]
#[macro_use]
extern crate sgx_tstd as std;

use std::vec::Vec;
use std::string::String;

#[macro_use]
extern crate sgx_tunittest;

use sgx_tunittest::*;
use sgx_types::*;
use secp256k1::{PublicKey, SecretKey};

mod test_ecies;
use test_ecies::*;

mod test_http_req;
use test_http_req::*;

#[no_mangle]
pub extern "C"
fn test_main_entrance() -> size_t {
    rsgx_unit_tests!(
        // ecies
        // utils.rs
        test_generate_keypair,
        test_aead_enc_then_dec,
        // test_http_req,
    )
}

use ecies::utils::{
    generate_keypair, encapsulate, decapsulate
};
use ecies::pure_aes::{aes_decrypt, aes_encrypt};
use ecies::{encrypt, decrypt};
use hdwallet_sgx::{
    ExtendedPrivKey, ExtendedPubKey,
    DefaultKeyChain, ChainPath, KeyChain
};
use http_req::request;

#[no_mangle]
pub extern "C" fn test_something() -> sgx_status_t {
    // let seed = hex::decode("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f").expect("decode");
    // let master_key = ExtendedPrivKey::with_seed(&seed).expect("master key");
    // let key_chain =
    //         DefaultKeyChain::new(master_key);
    // for chain_path in &["m", "m/0"] {
    //     let (key, _derivation) = key_chain.derive_private_key(ChainPath::from(*chain_path)).expect("fetch key");
    //     println!("sk: {:?}", key);

    //     let pub_key = ExtendedPubKey::from_private_key(&key);
    //     let pk_serialize_uncompressed = pub_key.public_key.serialize_uncompressed();
    //     println!("pk_serialize_uncompressed: {:?}", pk_serialize_uncompressed);
    //     let hexed = hex::encode(pk_serialize_uncompressed);
    //     println!("pk_serialize_uncompressed in hex: {}", hexed);
    // }
    println!("[+] test aes_enc_then_de...");
    // let plain_text: [u8; 5] = [0xde, 0xff, 0xab, 0xcd, 0x90];
    let plain_text = "12312312312".as_bytes();
    println!("plain  text: {:?}", plain_text);
    let mut buf = plain_text.to_vec();
    let receiver_pub = hex::decode("04e3b21e724ac35bb4a464544386d4538179a8f1836a13aa3f3495f425da97ba2a5c205521baab5f5fe84e205ee35ca4a803a3bd6059a642263d03393265b4e73b").unwrap();
    println!("receiver pub's len: {:?}", receiver_pub.len());
    let sk = hex::decode("800ebd7ff4c5db7dfbcbb3978dd2ccfe6313914534b0454d1c1772911a1b05a8").unwrap();

    let mut enc = encrypt(&receiver_pub, &mut buf).unwrap();
    println!("cipher text: {:?}", enc);
    let result = decrypt(&sk, &mut enc).unwrap();
    println!("result: {:?}", result);
    assert_eq!(&result[..], &plain_text[..]);

    let mut cipher_text: Vec<u8> = 
        vec![4,32,103,171,100,73,199,208,6,197,20,189,239,54,33,68,143,122,252,239,177,173,150,202,34,73,4,107,7,252,27,49,0,78,125,66,236,98,246,26,141,227,207,123,42,30,103,130,199,240,110,27,188,239,221,165,88,221,247,145,77,177,33,45,229,180,156,55,188,67,245,81,57,102,62,79,147,100,237,114,114,209,235,135,79,14,157,169,44,239,190,217];
    
    assert_eq!(enc.len(), cipher_text.len());

    let result = decrypt(&sk, &mut cipher_text).unwrap();
    println!("ret: {:?}", result);
    println!("[+] test enc_then_de success!");

    println!("[+] test string and vec...");
    let phone_number = "136711906587";
    let mut phone_number_buf = phone_number.as_bytes().to_vec();
    let mut phone_number_enc = encrypt(&receiver_pub, &mut phone_number_buf).unwrap();
    let phone_number_ret = decrypt(&sk, &mut phone_number_enc).unwrap();
    let pn = String::from_utf8(phone_number_ret).unwrap();
    println!("Phone number: {:?}", pn);
    println!("[+] test string and vec success!");

    println!("[+] test http req...");
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("http://www.job.sjtu.edu.cn", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("[+] test http req success!");

    sgx_status_t::SGX_SUCCESS
}