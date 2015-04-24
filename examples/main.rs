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

fn main() {
    unsafe {
        let ctx = groonga_init();
        let dbpath = "test.db";
        let c_dbpath = convert_str_to_cstr(dbpath);
        let path = Path::new(dbpath);
        let path_displayable = path.display();
        let db_ctx = if path.exists() {
            println!("{} exists create db skipped.", path_displayable);
            grn_db_open(ctx, c_dbpath);
        } else {
            grn_db_create(ctx, c_dbpath, None);
        };
        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let _ = groonga_fin(ctx);
    }
}
