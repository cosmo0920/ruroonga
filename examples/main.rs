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
        let command_length = command.len() as u32;
        println!("command length: {}", command.len());
        println!("command: {}", command);
        let c_command = convert_str_to_cstr(command);
        let flag = 0;
        let rc = groonga::grn_ctx_send(ctx, c_command, command_length, flag);
        let received_res: *mut *mut libc::c_char = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut *mut libc::c_char;
        let received_len: *mut libc::c_uint = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut libc::c_uint;
        let received_flag: *mut libc::c_int = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut libc::c_int;
        let _ = groonga::grn_ctx_recv(ctx, received_res, received_len, received_flag);
        /* for debug */
        let is_null_p = received_res.is_null();
        let is_null_p2 = received_len.is_null();
        let is_null_p3 = received_flag.is_null();
        println!("result str: {:?}", received_res);
        println!("result: {}", is_null_p);
        println!("result2: {}", is_null_p2);
        println!("result3: {}", is_null_p3);
        /* end */
        libc::free(received_res as *mut libc::c_void);
        libc::free(received_len as *mut libc::c_void);
        libc::free(received_flag as *mut libc::c_void);

        println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
        let _ = groonga_fin(ctx);
    }
}
