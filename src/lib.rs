#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

include!("./bindings.rs");

pub fn readconfig(path: Option<String>) -> Result<lirc_config, std::io::Error> {
    let c_path = path.map(|p| CString::new(p).unwrap());
    let c_str = match c_path {
        Some(p) => p.as_ptr(),
        None => std::ptr::null(),
    };

    unsafe {
        let mut raw = MaybeUninit::uninit();
        let ret = lirc_readconfig(c_str, raw.as_mut_ptr(), None);

        if ret != 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(std::ptr::read(raw.assume_init()))
    }
}

pub fn readconfig_only(path: Option<&str>) -> Result<lirc_config, i32> {
    let c_path = path.map(|p| CString::new(p).unwrap());
    let c_str = match c_path {
        Some(p) => p.as_ptr(),
        None => std::ptr::null(),
    };

    unsafe {
        let mut raw = MaybeUninit::uninit();

        let ret = lirc_readconfig_only(c_str, raw.as_mut_ptr(), None);
        if ret == -1 {
            return Err(-ret);
        }

        Ok(std::ptr::read(raw.assume_init()))
    }
}

pub fn freeconfig(mut conf: lirc_config) {
    unsafe {
        lirc_freeconfig(&mut conf);
    }
}

pub fn init(prog: &str, verbose: u32) -> Result<i32, std::io::Error> {
    unsafe {
        let prog_str = CString::new(prog).unwrap();
        let ret = lirc_init(prog_str.as_ptr(), verbose);
        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(ret)
    }
}

pub fn deinit() -> Result<(), std::io::Error> {
    unsafe {
        let ret = lirc_deinit();
        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
}

pub fn send_one(fd: i32, remote: &str, key: &str) -> Result<(), i32> {
    let r = std::ffi::CString::new(remote).unwrap();
    let k = std::ffi::CString::new(key).unwrap();

    unsafe {
        let ret = lirc_send_one(fd, r.as_ptr(), k.as_ptr());
        if ret != 0 {
            return Err(ret);
        }

        Ok(())
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

        let a = std::ffi::CStr::from_ptr(c.assume_init() as *mut ::std::os::raw::c_char).to_str();
        Ok(String::from(a.unwrap()))
    }
}

pub fn get_local_socket(path: &str, quiet: bool) -> Result<i32, std::io::Error> {
    let q = if quiet { 1 } else { 0 };
    let p = std::ffi::CString::new(path).unwrap();

    unsafe {
        let r = lirc_get_local_socket(p.as_ptr(), q);
        if r < 0 {
            return Err(std::io::Error::last_os_error());
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

pub fn set_mode(conf: &mut lirc_config, mode: &str) -> Result<String, std::io::Error> {
    unsafe {
        let m = std::ffi::CString::new(mode).expect("Failed to convert mode");

        let ret = lirc_setmode(conf, m.as_ptr());
        let ret_mode = CStr::from_ptr(ret);

        if ret_mode.to_str().unwrap().eq(mode) {
            return Err(std::io::Error::last_os_error());
        }

        Ok(ret_mode.to_str().unwrap().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_deinit() {
        let ret = init("Test", 1);
        assert!(ret.is_ok(), "{:?}", ret.err());
        assert!(ret.unwrap() > 0);

        let ret = deinit();
        assert!(ret.is_ok(), "{:?}", ret.err());

        let ret = init("Test1", 10);
        assert!(ret.is_ok(), "{:?}", ret.err());
        assert!(ret.unwrap() > 0);

        let ret = deinit();
        assert!(ret.is_ok(), "{:?}", ret.err());
    }
}
