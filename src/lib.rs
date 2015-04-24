#![feature(libc)]
#![feature(collections)]
#![feature(convert)]
#![feature(path_ext)]
extern crate libc;
use std::ffi::CStr;
use std::str;
use std::path::Path;
use std::fs::PathExt;

#[link(name = "groonga")]
extern {
    pub fn grn_init() -> i64;
    pub fn grn_ctx_open(rc: i64) -> *mut libc::c_void;
    pub fn grn_ctx_close(ctx: *mut libc::c_void) -> i64;
    pub fn grn_db_open(ctx: *mut libc::c_void, dbpath: *mut libc::c_char) -> *mut libc::c_void;
    pub fn grn_db_create(ctx: *mut libc::c_void, dbpath: *mut libc::c_char,
                         flag: Option<extern "C" fn(libc::c_int) -> libc::c_int>) -> *mut libc::c_void;
    pub fn grn_ctx_send(ctx: *mut libc::c_void, command: libc::c_char, command_length: libc::c_int,
                        flag: libc::c_int) -> i64;
    pub fn grn_ctx_recv(ctx: *mut libc::c_void, command: *mut libc::c_char, command_length: *mut libc::c_int,
                        flag: *mut libc::c_int) -> i64;
    pub fn grn_get_version() -> *mut libc::c_char;
    pub fn grn_fin() -> i64;
}

pub fn groonga_init() -> *mut libc::c_void {
    unsafe {
        let rc = grn_init();
        return grn_ctx_open(rc);
    }
}

pub fn groonga_fin(ctx: *mut libc::c_void) {
    unsafe {
        let _ = grn_ctx_close(ctx);
        let _ = grn_fin();
    }
}

pub fn groonga_db_use(ctx: *mut libc::c_void, dbpath: &str) -> *mut libc::c_void {
    unsafe {
        let c_dbpath = convert_str_to_cstr(dbpath);
        let path = Path::new(dbpath);
        let path_displayable = path.display();
        let db_ctx;
        if path.exists() {
            println!("{} exists create db skipped.", path_displayable);
            db_ctx = grn_db_open(ctx, c_dbpath);
        } else {
            db_ctx = grn_db_create(ctx, c_dbpath, None);
        };
        return db_ctx;
    }
}

pub fn get_groonga_version() -> &'static str {
    unsafe {
        return convert_cstr_to_str(grn_get_version());
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
