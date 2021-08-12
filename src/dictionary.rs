use crate::base::{CFTypeRef, CFTypeID, CFTypeRefWithBaseType, CFTypeRefAny, OpaqueCType};
use std::ffi::c_void;
use crate::prelude::*;

#[repr(C)]
pub struct CFDictionaryRef(OpaqueCType);
impl CFTypeRef for CFDictionaryRef {}
extern "C" {
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValue(theDict: *const CFDictionaryRef, key: *const c_void) -> *const CFTypeRefAny;
}

impl CFTypeRefWithBaseType for CFDictionaryRef {
    fn type_id() -> CFTypeID {
        unsafe { CFDictionaryGetTypeID() }
    }
}

impl CFDictionaryRef {
    pub fn get_with_ptr(&self, key: *const c_void) -> &CFTypeRefAny {
        unsafe{ &*CFDictionaryGetValue(self, key)}
    }

    pub unsafe fn get_with_key<K: CFTypeRef>(&self, key: &K) -> &CFTypeRefAny {
        self.get_with_ptr(key.as_ptr())
    }
}