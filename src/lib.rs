extern crate libc;
extern crate groonga_sys as ffi;

mod commandapi;
mod libgroonga;

pub use libgroonga::{LibGroonga, Context, Database, Command};
