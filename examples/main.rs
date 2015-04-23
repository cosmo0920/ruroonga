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
        let mut string = "test.db".to_string();
        let bytes = string.into_bytes();
        let mut x : Vec<libc::c_char> = bytes.map_in_place(|w| w as libc::c_char);;
        let slice = x.as_mut_slice();
        let dbpath = slice.as_mut_ptr();
        let db_ctx = grn_db_create(ctx, dbpath, None);
        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let _ = groonga_fin(ctx);
    }
}
