use crate::base::{CFTypeRef, CFTypeID, CFTypeRefWithBaseType, CFTypeRefAny};
use std::ffi::c_void;
use std::ops::Index;

#[repr(transparent)]
#[derive(Debug,Clone)]
pub struct CFDictionaryRef(*const c_void);
impl CFTypeRef for CFDictionaryRef {
    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr)}
}
extern "C" {
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *const c_void) -> CFTypeRefAny;
}

impl CFTypeRefWithBaseType for CFDictionaryRef {
    fn type_id() -> CFTypeID {
        unsafe { CFDictionaryGetTypeID() }
    }
}

impl CFDictionaryRef {
    ///Unsafe because return type is unmanaged
    pub unsafe fn get_with_ptr(&self, key: *const c_void) -> CFTypeRefAny {
        unsafe{ CFDictionaryGetValue(self.clone(), key) }
    }

    pub unsafe fn get_with_key<K: CFTypeRef>(&self, key: K) -> CFTypeRefAny {
        self.get_with_ptr(key.as_ptr())
    }
}