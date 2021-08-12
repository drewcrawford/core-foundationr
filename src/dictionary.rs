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
    ///Gets the given key, using raw pointers.
    ///
    /// The return value may be null.  In this case, either the key is not present in the dictionary,
    /// or it is present and has the explicit value NULL.
    pub unsafe fn get_with_ptr(&self, key: *const c_void) -> *const CFTypeAny {
        &*CFDictionaryGetValue(self, key)
    }

    ///Gets the given key, using some [CFType] key.
    ///
    /// The return value may be None.  In this case, either the key is not present in the dictionary,
    /// or it is present and has the explicit value NULL.
    /// The return value has the lifetime of the receiver
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