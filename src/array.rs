use std::ffi::c_void;
use crate::base::{CFTypeRef, CFTypeRefWithBaseType, CFTypeID, OpaqueCType};
#[repr(C)]
pub struct CFArrayRef(OpaqueCType);
impl CFTypeRef for CFArrayRef {
    fn as_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
    unsafe fn from_ptr(ptr: *const c_void) -> *const Self { ptr as *const Self }
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




