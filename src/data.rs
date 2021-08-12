use crate::base::{CFTypeRef, CFAllocatorRef, CFIndex, OpaqueCType};
use std::ffi::c_void;
use crate::cell::StrongCell;

extern "C" {
    fn CFDataCreate(allocator: *const CFAllocatorRef, bytes: *const u8, length: CFIndex) -> *const CFDataRef;
}

#[repr(C)]
pub struct CFDataRef(OpaqueCType);
impl CFTypeRef for CFDataRef {

    fn as_ptr(&self) -> *const c_void {
        self as *const Self as *const c_void
    }
    unsafe fn from_ptr(ptr: *const c_void) -> *const Self { ptr as *const Self }
}
impl CFDataRef {
    //- note: objc knows a faster path for owned strings
    //- note: uncertain about faster path for static strings?
    pub fn from_str(str: &str) -> StrongCell<CFDataRef> {
        let raw = unsafe{ CFDataCreate(CFAllocatorRef::null(), str.as_ptr(), str.as_bytes().len() as CFIndex) };
        unsafe{ StrongCell::assuming_retained(raw) }
    }
}

#[test] fn from_str() {
    use crate::prelude::*;
    let result = CFDataRef::from_str("hello world");
    let str = result.description().as_string();
    assert!(str.starts_with("<CFData"))
}