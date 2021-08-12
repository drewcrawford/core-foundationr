//!<CoreFoundation/base.h>


use std::ffi::c_void;
use std::os::raw::{c_long, c_ulong};
use crate::cell::StrongCell;

pub type CFOptionFlags = c_ulong;

pub type CFTypeID = c_ulong;

///For their pointers to be valid, our types have to be FFI-safe.
///
/// In particular, this means they cannot be ZSTs, which might not have a valid address.
/// Instead, we need to give them some non-ZST payload.  Note that this should never actually
/// be constructed, because we don't use 'owned' values of these types.
#[repr(C)]
pub(crate) struct OpaqueCType {
    _field: bool
}

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
}

#[repr(C)]
pub struct CFStringRef(OpaqueCType);
impl CFTypeRef for CFStringRef {}

extern "C" {
    //*c_void in here is basically CFTypeRef (which is a trait in Rust)
    fn CFCopyDescription(cf: *const c_void) -> *const CFStringRef;
    fn CFGetTypeID(cf: *const c_void ) -> CFTypeID;
}

pub trait CFTypeRefBehavior {
    fn description(&self) -> StrongCell<CFStringRef>;
    fn type_id(&self) -> CFTypeID;
    fn checked_cast<R: CFTypeRefWithBaseType>(&self) -> &R;
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
    unsafe fn from_ptr(ptr: *const c_void) -> *const Self;
}
impl<T: CFTypeRef> CFTypeRefBehavior for T {
    fn description(&self) -> StrongCell<CFStringRef> {
        let r1 = self.as_ptr();
        let raw = unsafe{ CFCopyDescription(r1) };
        unsafe{ StrongCell::assuming_retained(&*raw) }
    }
    fn type_id(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_ptr()) }
    }

    fn checked_cast<R: CFTypeRefWithBaseType>(&self) -> &R {
        assert_eq!(CFTypeRefBehavior::type_id(self),R::type_id());
        unsafe{ &*R::from_ptr(self.as_ptr()) }
    }

    fn as_ptr(&self) -> *const c_void {
        self as *const Self as *const c_void
    }

    unsafe fn from_ptr(ptr: *const c_void) -> *const Self {
        ptr as *const Self
    }
}

///These are types that have a "base type" (e.g. not polymorphic over underlying types).
pub trait CFTypeRefWithBaseType: CFTypeRef {
    fn type_id() -> CFTypeID;
}

#[repr(C)]
pub struct CFTypeRefAny(OpaqueCType);
impl CFTypeRef for CFTypeRefAny {}

#[repr(C)]
pub struct CFAllocatorRef(OpaqueCType);
impl CFAllocatorRef {
    pub fn null() -> *const CFAllocatorRef { std::ptr::null() as *const CFAllocatorRef }
}

pub type CFIndex = c_long;