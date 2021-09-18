
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
        test_generate_keypair,
    )
}