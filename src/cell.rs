use crate::base::{CFTypeRef, CFTypeRefWithBaseType, CFTypeRefBehavior};
use std::ffi::c_void;
use std::ops::Deref;
use std::any::Any;

#[derive(Debug)]
pub struct StrongCell<T: CFTypeRef>(T);
impl<T: CFTypeRef> StrongCell<T> {
    pub unsafe fn assuming_retained(t: T) -> Self {
        Self(t)
    }
    ///Perform a checked cast into the given type.
    ///
    /// This transfers ownership to the new type.
    ///
    pub unsafe fn cast_checked<R: CFTypeRefWithBaseType>(self) -> StrongCell<R> {
        assert_eq!(CFTypeRefBehavior::type_id(&self.0),R::type_id());
        //should be safe since both source and dst have the static lifetime (e.g. StrongCell)
        let new_type = unsafe{ R::from_ptr(self.as_ptr()) };
        let new_cell = unsafe{ StrongCell::assuming_retained(new_type)};
        std::mem::forget(self);
        new_cell
    }
}

extern "C" {
    fn CFRelease(type_ref: *const c_void);
}

impl<T: CFTypeRef> Drop for StrongCell<T> {
    fn drop(&mut self) {
        unsafe{ CFRelease(self.0.as_ptr()) };
    }
}
impl<T: CFTypeRef> Deref for StrongCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}