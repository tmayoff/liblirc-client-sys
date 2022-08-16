#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::{CString}, mem::MaybeUninit};

include!("./bindings.rs");

pub struct config {
    raw: MaybeUninit<lirc_config>,
}

impl config {
    pub fn new() -> Result<Self, i32> {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            lirc_readconfig(std::ptr::null(), &mut raw.as_mut_ptr(), None);
            Ok(config{raw})
        }
    }
}

impl Drop for config {
    fn drop (&mut self) {
        unsafe {
            lirc_freeconfig(self.raw.as_mut_ptr());
        }
    }
}

#[must_use]
pub fn init(prog: &str, verbose: u32) -> i32{
    unsafe {
        let prog_str = CString::new(prog).unwrap().into_raw();
        lirc_init(prog_str, verbose)
    }
}

#[must_use]
pub fn deinit() -> i32{
    unsafe {
        lirc_deinit()
    }
}

pub fn nextcode() -> Result<String, i32> {
    unsafe {
        let mut ptr = MaybeUninit::uninit();
        let ret = lirc_nextcode( &mut ptr.as_mut_ptr());
        if ret != 0 {
            return Err(ret);
        }

        let r = std::ffi::CStr::from_ptr(ptr.as_mut_ptr()).to_str();
        Ok(String::from(r.unwrap()))
    }
}

pub fn code2char(conf: config, code: String) -> Result<String, i32> {
    unsafe {
        let mut c = MaybeUninit::uninit();
        let ret = lirc_code2char(config.raw, code.as_ptr(), c);
        if ret != 0 {
            Err(ret);
        }
        let r = std::ffi::CStr::from_ptr(c.as_mut_ptr()).to_str();
        Ok(String::from(r.unwrap()))
    }
}