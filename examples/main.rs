extern crate ruroonga;
use ruroonga::*;
use std::ffi::CStr;
use std::str;

fn main() {
    unsafe {
        let ctx = groonga_init();
        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let _ = groonga_fin(ctx);
    }
}
