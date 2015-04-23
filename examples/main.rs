extern crate ruroonga;
use ruroonga::*;
use std::ffi::CStr;
use std::str;

fn main() {
    unsafe {
        let rc = grn_init();
        let ctx = grn_ctx_open(rc);
        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let rc = grn_ctx_close(ctx);
        let _ = grn_fin();
    }
}
