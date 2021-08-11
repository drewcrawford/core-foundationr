use crate::base::{CFTypeRef, CFAllocatorRef, CFIndex};
use std::ffi::c_void;
use crate::cell::StrongCell;
use std::marker::PhantomData;

extern "C" {
    //not actually static
    fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef<'static>;
}

#[repr(transparent)]
#[derive(Debug,Clone)]
pub struct CFDataRef<'a>(*const c_void,PhantomData<&'a ()>);
impl<'a> CFTypeRef for CFDataRef<'a> {

    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr, PhantomData::default())}
}
impl<'a> CFDataRef<'a> {
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