use crate::base::{CFType, CFTypeWithBaseType, CFTypeBehavior, CFTypeAny};
use std::ffi::c_void;
use std::ops::Deref;
use std::fmt::Formatter;

///A 'smart pointer' that keeps a strong reference to the CF object.
///
/// The object will be released when the `StrongCell` is dropped.
pub struct StrongCell<T: CFType>(*const T);
impl<T: CFType> StrongCell<T> {
    ///Creates a [StrongCell], assuming the pointer is retained already (so the conversion is a no-op).
    ///
    ///This is unsafe, because there's no way to check if it's retained or even valid
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
impl <T: std::fmt::Display + CFType> std::fmt::Display for StrongCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //I think this is fine because of the precedent in Drop.
        // todo: Maybe we should convert the inner pointer to NonNull...
        let as_ref = unsafe {&*self.0};
        f.write_fmt(format_args!("{}",as_ref))
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