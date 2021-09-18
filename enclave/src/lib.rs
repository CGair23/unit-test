
#![no_std]
#[macro_use]
extern crate sgx_tstd as std;

use std::vec::Vec;
use std::string::String;

#[macro_use]
extern crate sgx_tunittest;

use sgx_tunittest::*;
use sgx_types::*;

mod test_ecies;
use test_ecies::*;

#[no_mangle]
pub extern "C"
fn test_main_entrance() -> size_t {
    rsgx_unit_tests!(
        // ecies
        // utils.rs
        test_generate_keypair,
    )
}

use ecies::utils::{
    generate_keypair, encapsulate
};

#[no_mangle]
pub extern "C" fn test_something() -> sgx_status_t {
    let (sk1, pk1) = generate_keypair();
    let (sk2, pk2) = generate_keypair();

    println!("[+] test encapsulate()...");
    let out = encapsulate(&sk1, &pk2).unwrap();
    println!("{:?}", out);

    sgx_status_t::SGX_SUCCESS
}