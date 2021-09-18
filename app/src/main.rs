use sgx_types::*;
use sgx_urts::SgxEnclave;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
    fn test_main_entrance(eid: sgx_enclave_id_t, retval: *mut size_t) -> sgx_status_t;
}

extern {
    fn test_something(eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let mut retval = 0usize;

    let result = unsafe {
        test_main_entrance(enclave.geteid(),
                           &mut retval)
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    assert_eq!(retval, 0);

    println!("[+] unit_test ended!");

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let result = unsafe {
        test_something(enclave.geteid(),
                      &mut retval
                    )
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    println!("[+] test_something success...");

    enclave.destroy();
}
