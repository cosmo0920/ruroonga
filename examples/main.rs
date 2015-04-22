extern crate ruroonga;
use ruroonga::*;
use std::ffi::CStr;
use std::str;

fn main() {
    unsafe {
        let rc = grn_init();
        let ctx = grn_ctx_open(rc);
        let slice = CStr::from_ptr(ruroonga::grn_get_version());
        println!("Hello in Ruroonga with Groonga: {}", str::from_utf8(slice.to_bytes()).unwrap());
        let _ = grn_fin();
    }
}
