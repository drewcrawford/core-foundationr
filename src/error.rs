use std::ffi::c_void;
use crate::base::CFTypeRef;

#[repr(transparent)]
#[derive(Debug,Clone)]
pub struct CFErrorRef(*const c_void);
impl CFTypeRef for CFErrorRef {
    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr)}
}