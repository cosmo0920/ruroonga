#![feature(libc)]
#![feature(collections)]
#![feature(convert)]
#![feature(path_ext)]
extern crate ruroonga;
extern crate libc;
use ruroonga::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::string::String;
use std::path::Path;
use std::fs::PathExt;
use std::mem;

fn main() {
    unsafe {
        let ctx = groonga_init();
        let dbpath = "test.db";
        let db_ctx = groonga_db_use(ctx, dbpath);
        let command = "table_create Users TABLE_HASH_KEY ShortText";
        let _ = groonga_execute_command(ctx, command);
        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let _ = groonga_fin(ctx);
    }
}
