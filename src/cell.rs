use crate::base::{CFType, CFTypeWithBaseType, CFTypeBehavior, CFTypeAny};
use std::ffi::c_void;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Display, Formatter};
use std::mem::forget;
use std::ptr::NonNull;

///A 'smart pointer' that keeps a strong reference to the CF object.
///
/// The object will be released when the `StrongCell` is dropped.
///
/// For a mutable version see [StrongMutCell].
pub struct StrongCell<T: CFType>(NonNull<T>);
impl<T: CFType> StrongCell<T> {
    ///Creates a [StrongCell], assuming the pointer is retained already (so the conversion is a no-op) and non-null.
    ///
    ///This is unsafe, because there's no way to check if it's retained or even valid
    pub unsafe fn assuming_retained_nonnull(t: *const T) -> Self {
        Self(NonNull::new_unchecked(t as *mut T))
    }
    ///Perform a checked cast into the given type.
    ///
    /// This transfers ownership to the new type.
    pub fn cast_checked<R: CFTypeWithBaseType>(self) -> StrongCell<R> {
        let any = unsafe{ CFTypeAny::from_ptr(self.0.as_ptr() as *const c_void) };
        assert_eq!(CFTypeBehavior::type_id(unsafe{ &*any}), R::type_id());
        //should be safe since both source and dst have the static lifetime (e.g. StrongCell)
        let new_type = unsafe{ R::from_ptr(self.as_ptr()) };
        let new_cell = unsafe{ StrongCell::assuming_retained_nonnull(new_type)};
        std::mem::forget(self);
        new_cell
    }
    /**
    Retains the passed pointer.

    # Safety
    Assumes the pointer is non-null.
    */
    pub unsafe fn retain_assuming_nonnull(t: *const T) -> Self {
        CFRetain(t as *const c_void);
        Self(NonNull::new_unchecked(t as *mut T))
    }
    /**
    Casts to mutable type.

    # Safety
    The object must be mutable, "whatever that means".  You are also guaranteeing that nobody
    else is has a mutable reference to the object.
    */
    pub unsafe fn assuming_mut(self) -> StrongMutCell<T> {
        let s = StrongMutCell(self.0);
        forget(self);
        s
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
        let as_ref = unsafe {&*self.0.as_ptr()};
        f.write_fmt(format_args!("{}",as_ref))
    }
}

extern "C" {
    fn CFRelease(type_ref: *const c_void);
    fn CFRetain(type_ref: *const c_void);
}

impl<T: CFType> Drop for StrongCell<T> {
    fn drop(&mut self) {
        unsafe{ CFRelease(self.0.as_ptr() as *const c_void) };
    }
}
impl<T: CFType> Deref for StrongCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0.as_ptr() }
    }
}

/**
Like [StrongCell], but mutable.

To create one, see [StrongCell::assuming_mut].
*/
pub struct StrongMutCell<T: CFType>(NonNull<T>);

impl<T: CFType> Debug for StrongMutCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StrongMutCell<{}>({:?})",stringify!(CFError),self.0))
    }
}
impl<T: CFType + Display> Display for StrongMutCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //I think this is fine because of the precedent in Drop.
        let as_ref = unsafe {&*self.0.as_ptr()};
        f.write_fmt(format_args!("{}",as_ref))
    }
}

impl<T: CFType> Drop for StrongMutCell<T> {
    fn drop(&mut self) {
        unsafe{ CFRelease(self.0.as_ptr() as *const c_void) };
    }
}

impl<T: CFType> Deref for StrongMutCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0.as_ptr() }
    }
}
impl<T: CFType> DerefMut for StrongMutCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{ &mut *self.0.as_ptr() }
    }
}