use crate::base::{CFTypeRef, CFAllocatorRef, CFIndex, OpaqueCType};
use crate::cell::StrongCell;

extern "C" {
    fn CFDataCreate(allocator: *const CFAllocatorRef, bytes: *const u8, length: CFIndex) -> *const CFDataRef;
}

#[repr(C)]
pub struct CFDataRef(OpaqueCType);
impl CFTypeRef for CFDataRef {}
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