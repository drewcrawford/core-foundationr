use crate::base::{CFType, CFTypeID, CFTypeWithBaseType, CFTypeAny, OpaqueCType};
use std::ffi::c_void;
use crate::CFTypeBehavior;

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
    pub fn get_with_ptr(&self, key: *const c_void) -> *const CFTypeAny {
        unsafe{ &*CFDictionaryGetValue(self, key)}
    }

    pub fn get_with_key<K: CFType>(&self, key: &K) -> Option<&CFTypeAny> {
        unsafe {
            let result = self.get_with_ptr(key.as_ptr());
            if result.is_null() {
                None
            }
            else {
                Some(&*result)
            }
        }
    }
}