extern crate libc;
extern crate groonga_sys as ffi;

use std::ffi::{CString, CStr};
use std::str;
use std::path::Path;
use std::ptr;
use std::mem;
use std::result::Result::{Ok, Err};
use ffi::groonga;

macro_rules! convert_cstr_to_str {
    ($cstr:expr) => {
        {
            let slice = CStr::from_ptr($cstr);
            let result = str::from_utf8(slice.to_bytes());
            match result {
                Ok(v) => return v,
                Err(_) => return "",
            }
        }
    };
}

/// Initialize libgroonga.
pub fn groonga_init() -> groonga::grn_rc {
    unsafe {
        return groonga::grn_init();
    }
}

/// Finalized libgroonga.
pub fn groonga_fin() -> groonga::grn_rc {
    unsafe {
        return groonga::grn_fin();
    }
}

/// Open libgroonga context.
pub fn groonga_ctx_open(rc: libc::c_int) -> *mut groonga::grn_ctx {
    unsafe {
        return groonga::grn_ctx_open(rc);
    }
}

/// Close libgroonga context.
pub fn groonga_ctx_close(ctx: *mut groonga::grn_ctx) -> groonga::grn_rc {
    unsafe {
        return groonga::grn_ctx_close(ctx);
    }
}

/// Create/Open libgronga database.
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

/// Send and receive libgroonga command.
pub fn groonga_execute_command(ctx: *mut groonga::grn_ctx,
                               command: &str)
                               -> Result<String, String> {
    unsafe {
        let command_length = command.len() as u32;
        let c_command = convert_str_to_cstr(command);
        let flag = 0;
        let _ = groonga::grn_ctx_send(ctx, c_command, command_length, flag);
        let mut output_ptr: *mut libc::c_char = ptr::null_mut();
        let received_len: *mut libc::c_uint =
            libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut libc::c_uint;
        let received_flag: *mut libc::c_int =
            libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut libc::c_int;
        let _ = groonga::grn_ctx_recv(ctx, &mut output_ptr, received_len, received_flag);
        if output_ptr.is_null() {
            return Err("Couldn't get result.".to_string());
        }
        let out = CStr::from_ptr(&mut *output_ptr).to_string_lossy().into_owned();
        libc::free(received_len as *mut libc::c_void);
        libc::free(received_flag as *mut libc::c_void);
        return Ok(out);
    }
}

/// Get libgroonga version information.
pub fn get_groonga_version() -> &'static str {
    unsafe {
        return convert_cstr_to_str!(groonga::grn_get_version());
    }
}

pub fn convert_str_to_cstr(s: &str) -> *mut libc::c_char {
    let cstr = CString::new(s).unwrap();
    let ptr = cstr.into_raw();
    return ptr;
}
