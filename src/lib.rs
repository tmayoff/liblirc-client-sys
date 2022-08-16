#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CString};
use Result;

include!("./bindings.rs");

pub struct lirc_config {
    raw: *mut lirc_config_raw,
}

impl lirc_config {
    pub fn new() -> Result<Self, i32> {
        unsafe {
            let mut raw: *mut lirc_config_raw = std::mem::uninitialized();
            lirc_readconfig(std::ptr::null(), &mut raw, None);
            return Ok(lirc_config{raw});
        }
    }
}

impl Drop for lirc_config {
    fn drop (&mut self) {
        unsafe {
            lirc_freeconfig(self.raw);
        }
    }
}

#[must_use]
pub fn init(prog: &str, verbose: u32) -> i32{
    unsafe {
        let prog_str = CString::new(prog).unwrap().into_raw();
        return lirc_init(prog_str, verbose);
    }
}

#[must_use]
pub fn deinit() -> i32{
    unsafe {
        return lirc_deinit();
    }
}

#[must_use]
pub fn nextcode(code: &String) -> i32 {
    unsafe {
        let c = CString::new(prog).unwrap().into_raw();
        lirc_nextcode(c);
    }
}