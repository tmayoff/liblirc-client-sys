#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::{CString}, mem::MaybeUninit};

include!("./bindings.rs");

pub fn readconfig(_path: Option<&str>) -> Result<lirc_config, i32> {
    unsafe {
        let mut raw = MaybeUninit::uninit();

        // let ret: i32;
        // if path.is_some() {
        //     ret = lirc_readconfig(path.expect("None").as_ptr(), raw.as_mut_ptr(), None);
        // } else {
        let ret = lirc_readconfig(std::ptr::null(), raw.as_mut_ptr(), None);
        // }

        if ret != 0 {
            return Err(ret);
        }

        Ok(std::ptr::read(raw.assume_init()))
    }
}

pub fn freeconfig(mut conf: lirc_config) {
    unsafe {
        lirc_freeconfig(&mut conf);
    }
}

#[must_use]
pub fn init(prog: &str, verbose: u32) -> i32{
    unsafe {
        let prog_str = CString::new(prog).unwrap();
        lirc_init(prog_str.as_ptr(), verbose)
    }
}

#[must_use]
pub fn deinit() -> i32{
    unsafe {
        lirc_deinit()
    }
}

pub fn send_one(fd: i32, remote: &str, key: &str) -> i32 {
    unsafe {
        let r = std::ffi::CString::new(remote).unwrap();
        let k = std::ffi::CString::new(key).unwrap();
        lirc_send_one(fd, r.as_ptr(), k.as_ptr())
    }
}

pub fn nextcode() -> Result<String, i32> {
    unsafe {
        let mut str_buf = MaybeUninit::uninit();
        let ret = lirc_nextcode(str_buf.as_mut_ptr());
        if ret != 0 {
            return Err(ret);
        }

        let r = std::ffi::CStr::from_ptr(str_buf.assume_init()).to_str();
        Ok(String::from(r.unwrap()))
    }
}

pub fn code2char(mut conf: lirc_config, code: &mut str) -> Result<String, i32> {
    unsafe {
        let mut c = MaybeUninit::uninit();
        let ret = lirc_code2char(&mut conf, code.as_mut_ptr(), c.as_mut_ptr());
        if ret != 0 {
            return Err(ret);
        }

        let a = std::ffi::CStr::from_ptr(c.assume_init() as *mut i8).to_str();
        Ok(String::from(a.unwrap()))
    }
}

pub fn get_local_socket(path: &str, quiet: bool) -> Result<i32, i32> {
    unsafe {
        let q = if quiet { 1 } else { 0 };
        let p = std::ffi::CString::new(path).unwrap();
        let r = lirc_get_local_socket(p.as_ptr(), q);
        if r < 0 {
            return Err(r);
        }

        Ok(r)
    }
}

pub fn get_remote_socket(host: &str, port: i32, quiet: bool) -> Result<i32, i32> {
    unsafe {
        let q: i32 = if quiet { 1 } else { 0 };
        let h = std::ffi::CString::new(host).unwrap();
        let r = lirc_get_remote_socket(h.as_ptr(), port, q);
        if r < 0 {
            return Err(r);
        }

        Ok(r)
    }
}