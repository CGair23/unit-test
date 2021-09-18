
#!no_std]
#[macro_use]
extern crate sgx_tstd as std;

#[macro_use]
extern crate sgx_tunittest;

use sgx_tunittest::*;

#[no_mangle]
pub extern "C"
fn test_main_entrance() -> size_t {
    rsgx_unit_tests!()
}