use http_req::request;
use std::prelude::v1::*;

pub fn test_http_req() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("https://doc.rust-lang.org/", &mut writer).unwrap();

    assert_eq!(res.status_code().is_success(), true);
}