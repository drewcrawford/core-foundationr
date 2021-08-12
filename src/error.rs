use std::ffi::c_void;
use crate::base::{CFTypeRef, OpaqueCType};

#[repr(C)]
pub struct CFErrorRef(OpaqueCType);
impl CFTypeRef for CFErrorRef {
    fn as_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
    unsafe fn from_ptr(ptr: *const c_void) -> *const Self { ptr as *const Self }
}