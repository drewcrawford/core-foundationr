use std::ffi::c_void;
use crate::base::{CFTypeRef, CFTypeRefWithBaseType, CFTypeID};

#[derive(Debug,Clone,Copy)]
pub struct CFArrayRef(*const c_void);
impl CFTypeRef for CFArrayRef {
    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr)}
}
extern "C" {
    fn CFArrayGetTypeID() -> CFTypeID;
}
impl CFTypeRefWithBaseType for CFArrayRef {
    fn type_id() -> CFTypeID {
        unsafe { CFArrayGetTypeID() }
    }
}

struct CFArrayRefIterator {

}




