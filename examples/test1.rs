
use std::ffi::CString;
use libc::{open, PROT_READ};
// extern crate test_ndk;
use test_ndk::{hook_open};

fn main() {
    hook_open();
    unsafe { open(CString::new("/data/").unwrap().into_raw(), PROT_READ); }
}