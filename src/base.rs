//!<CoreFoundation/base.h>


use std::ffi::c_void;
use std::os::raw::{c_long, c_ulong};
use crate::cell::StrongCell;
use std::any::Any;
use crate::dictionary::CFDictionaryRef;

pub type CFOptionFlags = c_ulong;

pub type CFTypeID = c_ulong;

#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex
}

///We choose to represent CFTypeRef as a trait.
///
/// See also:
/// * CFTypeRefAny - struct with no associated type
pub trait CFTypeRef {
    fn as_ptr(&self) -> *const c_void;
    ///Create a type from a raw pointer
    ///
    /// # Safety
    /// **WARNING**.  The value returned from this function is only valid for the lifetime of the corresponding objc reference.
    /// This invariant is **not** enforced by the Rust borrow checker.  Ergo, you must enforce it.
    ///
    /// If you fail, 'safe' functions involving this return type may have UB, which is very scary.
    ///
    /// If you do not know what you're doing, put the return value into a [StrongCell] right away to
    /// promote to the `'static` lifetime.  Such use should be safe, at some additional performance cost.
    unsafe fn from_ptr(ptr: *const c_void) -> Self;
}

#[repr(transparent)]
#[derive(Debug,Clone)]
pub struct CFStringRef(*const c_void);
impl CFTypeRef for CFStringRef {
    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr)}
}

extern "C" {
    //*c_void in here is basically CFTypeRef (which is a trait in Rust)
    fn CFCopyDescription(cf: *const c_void) -> CFStringRef;
    fn CFGetTypeID(cf: *const c_void ) -> CFTypeID;
}

pub trait CFTypeRefBehavior {
    fn description(&self) -> StrongCell<CFStringRef>;
    fn type_id(&self) -> CFTypeID;
    fn checked_cast<R: CFTypeRefWithBaseType>(self) -> R;
}
impl<T: CFTypeRef> CFTypeRefBehavior for T {
    fn description(&self) -> StrongCell<CFStringRef> {
        let raw = unsafe{ CFCopyDescription(self.as_ptr()) };
        unsafe{ StrongCell::assuming_retained(raw) }
    }
    fn type_id(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_ptr()) }
    }

    fn checked_cast<R: CFTypeRefWithBaseType>(self) -> R {
        assert_eq!(CFTypeRefBehavior::type_id(&self),R::type_id());
        unsafe{ R::from_ptr(self.as_ptr()) }
    }
}

///These are types that have a "base type" (e.g. not polymorphic over underlying types).
pub trait CFTypeRefWithBaseType: CFTypeRef {
    fn type_id() -> CFTypeID;
}

#[derive(Debug,Clone)]
pub struct CFTypeRefAny(*const c_void);
impl CFTypeRef for CFTypeRefAny {
    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr)}
}

#[repr(transparent)]
#[derive(Debug,Clone)]
pub struct CFAllocatorRef(*const c_void);
impl CFAllocatorRef {
    pub fn null() -> CFAllocatorRef { Self (std::ptr::null() )}
}

pub type CFIndex = c_long;