extern crate pkg_config;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let lib_dir = env::var("GROONGA_LIB_DIR").ok();
    let bin_dir = env::var("GROONGA_BIN_DIR").ok();
    let include_dir = env::var("GROONGA_INCLUDE_DIR").ok();

    if lib_dir.is_none() && include_dir.is_none() {
        if !target.contains("windows") {
            if let Ok(info) = pkg_config::find_library("groonga") {
                if info.include_paths.len() > 0 {
                    let paths = env::join_paths(info.include_paths).unwrap();
                    println!("cargo:include={}", paths.to_str().unwrap());
                }
                return;
            }
        }
    }

    let lib = "groonga";
    let mode = if env::var_os("GROONGA_STATIC").is_some() {
        "static"
    } else {
        "dylib"
    };

    if let Some(lib_dir) = lib_dir {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    }
    if let Some(bin_dir) = bin_dir {
        println!("cargo:rustc-link-search=native={}", bin_dir);
    }

    println!("cargo:rustc-link-lib={}={}", mode, lib);

    if let Some(include_dir) = include_dir {
        println!("cargo:include={}", include_dir);
    }
}
