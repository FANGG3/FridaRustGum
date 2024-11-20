
use std::ffi::CString;
use libc::{open, PROT_READ};
use log::info;
// extern crate test_ndk;
use test_ndk::{hook_open};
use test_ndk::stalker;

fn main() {
    
    unsafe { 
        // hook_open();
        open(CString::new("/data/").unwrap().into_raw(), PROT_READ); 
        
    }

}