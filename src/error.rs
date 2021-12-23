use std::fmt::{Formatter};
use std::ops::Deref;
use crate::base::{CFIndex, CFType, OpaqueCType};
use crate::{CFString, StrongCell};

#[repr(C)]
#[derive(Debug)]
pub struct CFError(OpaqueCType);
impl CFType for CFError {}

pub type CFErrorDomain = CFString;

extern "C" {
    fn CFErrorCopyDescription(error: *const CFError) -> *const CFString;
}

impl std::fmt::Display for CFError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let description = CFErrorCopyDescription(self);
            //documented to never be null.
            let description_ref = StrongCell::assuming_retained(description);
            //todo: could potentially be optimized with an internal pointer perhaps
            f.write_fmt(format_args!("{}",description_ref.as_string()))
        }
    }
}
impl std::error::Error for CFError {}

#[test] fn description() {
    use std::ffi::c_void;

    extern "C" {
        fn CFErrorCreate(allocator: *const c_void, domain: *const CFErrorDomain, code: CFIndex, user_info: *const c_void) -> *const CFError;
    }

    unsafe {
        let domain = CFString::from_str("test");
        let error = CFErrorCreate(std::ptr::null(), &*domain, 0,std::ptr::null());
        let error_ref = StrongCell::assuming_retained(error);
        let error = format!("{}",error_ref.deref());
        println!("{}",error);
        assert_eq!(error,"The operation couldn’t be completed. (test error 0.)")
    }
}