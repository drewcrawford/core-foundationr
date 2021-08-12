use crate::base::{CFType, CFTypeWithBaseType, CFTypeBehavior, CFTypeAny};
use std::ffi::c_void;
use std::ops::Deref;
use std::fmt::Formatter;

pub struct StrongCell<T: CFType>(*const T);
impl<T: CFType> StrongCell<T> {
    pub unsafe fn assuming_retained(t: *const T) -> Self {
        Self(t as *const T)
    }
    ///Perform a checked cast into the given type.
    ///
    /// This transfers ownership to the new type.
    pub fn cast_checked<R: CFTypeWithBaseType>(self) -> StrongCell<R> {
        let any = unsafe{ CFTypeAny::from_ptr(self.0 as *const c_void) };
        assert_eq!(CFTypeBehavior::type_id(unsafe{ &*any}), R::type_id());
        //should be safe since both source and dst have the static lifetime (e.g. StrongCell)
        let new_type = unsafe{ R::from_ptr(self.as_ptr()) };
        let new_cell = unsafe{ StrongCell::assuming_retained(new_type)};
        std::mem::forget(self);
        new_cell
    }
}
impl<T: CFType> std::fmt::Debug for StrongCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StrongCell<{}>({:?})",stringify!(T),self.0))
    }
}

extern "C" {
    fn CFRelease(type_ref: *const c_void);
}

impl<T: CFType> Drop for StrongCell<T> {
    fn drop(&mut self) {
        unsafe{ CFRelease(self.0 as *const c_void) };
    }
}
impl<T: CFType> Deref for StrongCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0 }
    }
}