use crate::base::{CFType, CFTypeID, CFTypeWithBaseType, CFTypeAny, OpaqueCType};
use std::ffi::c_void;
use crate::prelude::*;

#[repr(C)]
pub struct CFDictionary(OpaqueCType);
impl CFType for CFDictionary {}
extern "C" {
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValue(theDict: *const CFDictionary, key: *const c_void) -> *const CFTypeAny;
}

impl CFTypeWithBaseType for CFDictionary {
    fn type_id() -> CFTypeID {
        unsafe { CFDictionaryGetTypeID() }
    }
}

impl CFDictionary {
    pub fn get_with_ptr(&self, key: *const c_void) -> &CFTypeAny {
        unsafe{ &*CFDictionaryGetValue(self, key)}
    }

    pub unsafe fn get_with_key<K: CFType>(&self, key: &K) -> &CFTypeAny {
        self.get_with_ptr(key.as_ptr())
    }
}