#![feature(libc)]
#![feature(collections)]
#![feature(convert)]
#![feature(path_ext)]
extern crate libc;
use std::ffi::CStr;
use std::str;
use std::path::Path;
use std::fs::PathExt;
use std::mem;

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
            // TODO: suspicious implementation. It is really correct?
            db_ctx = groonga::grn_db_create(ctx, c_dbpath, ::std::mem::zeroed());
        };
        return db_ctx;
    }
}

pub fn groonga_execute_command(ctx: *mut groonga::grn_ctx, command: &str) -> libc::c_uint
{
    unsafe {
        let command_length = command.len() as u32;
        let c_command = convert_str_to_cstr(command);
        let flag = 0;
        let _ = groonga::grn_ctx_send(ctx, c_command, command_length, flag);
        let received_res: *mut *mut libc::c_char = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut *mut libc::c_char;
        let received_len: *mut libc::c_uint = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut libc::c_uint;
        let received_flag: *mut libc::c_int = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut libc::c_int;
        let rc = groonga::grn_ctx_recv(ctx, received_res, received_len, received_flag);
        libc::free(received_res as *mut libc::c_void);
        libc::free(received_len as *mut libc::c_void);
        libc::free(received_flag as *mut libc::c_void);
        return rc;
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
