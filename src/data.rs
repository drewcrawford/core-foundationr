use crate::base::{CFType, CFAllocator, CFIndex, OpaqueCType};
use crate::cell::StrongCell;

extern "C" {
    fn CFDataCreate(allocator: *const CFAllocator, bytes: *const u8, length: CFIndex) -> *const CFData;
}

#[repr(C)]
pub struct CFData(OpaqueCType);
impl CFType for CFData {}
impl CFData {
    //- note: objc knows a faster path for owned strings
    //- note: uncertain about faster path for static strings?
    pub fn from_str(str: &str) -> StrongCell<CFData> {
        let raw = unsafe{ CFDataCreate(CFAllocator::null(), str.as_ptr(), str.as_bytes().len() as CFIndex) };
        unsafe{ StrongCell::assuming_retained(raw) }
    }
}

#[test] fn from_str() {
    use crate::prelude::*;
    let result = CFData::from_str("hello world");
    let str = result.description().as_string();
    assert!(str.starts_with("<CFData"))
}