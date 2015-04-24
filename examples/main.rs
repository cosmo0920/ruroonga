#![feature(libc)]
#![feature(collections)]
#![feature(convert)]
extern crate ruroonga;
extern crate libc;
use ruroonga::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::string::String;

fn main() {
    unsafe {
        let ctx = groonga_init();
        let dbpath = "test.db";
        let c_dbpath = convert_str_to_cstr(dbpath);
        let db_ctx = grn_db_create(ctx, c_dbpath, None);
        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let _ = groonga_fin(ctx);
    }
}
