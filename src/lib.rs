#![feature(libc)]
#![feature(collections)]
#![feature(convert)]
#![feature(path_ext)]
extern crate libc;
use std::ffi::CStr;
use std::str;
use std::path::Path;
use std::fs::PathExt;

pub mod groonga;

pub fn groonga_init() -> *mut groonga::grn_ctx {
    unsafe {
        let rc = groonga::grn_init();
        return groonga::grn_ctx_open(rc);
    }
}

pub fn groonga_fin(ctx: *mut groonga::grn_ctx) {
    unsafe {
        let _ = groonga::grn_ctx_close(ctx);
        let _ = groonga::grn_fin();
    }
}

pub fn groonga_db_use(ctx: *mut groonga::grn_ctx, dbpath: &str) -> *mut groonga::grn_obj {
    unsafe {
        let c_dbpath = convert_str_to_cstr(dbpath);
        let path = Path::new(dbpath);
        let path_displayable = path.display();
        let db_ctx;
        if path.exists() {
            println!("{} exists. Creating db is skipped.", path_displayable);
            db_ctx = groonga::grn_db_open(ctx, c_dbpath);
        } else {
            db_ctx = groonga::grn_db_create(ctx, c_dbpath, ::std::mem::zeroed());
        };
        return db_ctx;
    }
}

pub fn get_groonga_version() -> &'static str {
    unsafe {
        return convert_const_cstr_to_str(groonga::grn_get_version());
    }
}

pub fn convert_const_cstr_to_str(cstr: *const libc::c_char) -> &'static str {
    unsafe {
        let slice = CStr::from_ptr(cstr);
        return str::from_utf8(slice.to_bytes()).unwrap();
    }
}

pub fn convert_cstr_to_str(cstr: *mut libc::c_char) -> &'static str {
    unsafe {
        let slice = CStr::from_ptr(cstr);
        return str::from_utf8(slice.to_bytes()).unwrap();
    }
}

pub fn convert_str_to_cstr(s: &str) -> *mut libc::c_char {
    let string = s.to_string();
    let bytes = string.into_bytes();
    let mut x : Vec<libc::c_char> = bytes.map_in_place(|w| w as libc::c_char);;
    let slice = x.as_mut_slice();
    return slice.as_mut_ptr();
}
