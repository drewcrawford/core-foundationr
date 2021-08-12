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
pub trait CFType {
}

#[repr(C)]
pub struct CFString(OpaqueCType);
impl CFType for CFString {}
impl CFTypeWithBaseType for CFString {
    fn type_id() -> CFTypeID {
        unsafe {CFStringGetTypeID()}
    }
}

extern "C" {
    //*c_void in here is basically CFTypeRef (which is a trait in Rust)
    fn CFCopyDescription(cf: *const c_void) -> *const CFString;
    fn CFGetTypeID(cf: *const c_void ) -> CFTypeID;
    fn CFStringGetTypeID() -> CFTypeID;
}

pub trait CFTypeBehavior {
    fn description(&self) -> StrongCell<CFString>;
    fn type_id(&self) -> CFTypeID;
    fn checked_cast<R: CFTypeWithBaseType>(&self) -> &R;
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
impl<T: CFType> CFTypeBehavior for T {
    fn description(&self) -> StrongCell<CFString> {
        let r1 = self.as_ptr();
        let raw = unsafe{ CFCopyDescription(r1) };
        unsafe{ StrongCell::assuming_retained(&*raw) }
    }
    fn type_id(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_ptr()) }
    }

    fn checked_cast<R: CFTypeWithBaseType>(&self) -> &R {
        assert_eq!(CFTypeBehavior::type_id(self), R::type_id());
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
pub trait CFTypeWithBaseType: CFType {
    fn type_id() -> CFTypeID;
}

#[repr(C)]
pub struct CFTypeAny(OpaqueCType);
impl CFType for CFTypeAny {}

#[repr(C)]
pub struct CFAllocator(OpaqueCType);
impl CFAllocator {
    pub fn null() -> *const CFAllocator { std::ptr::null() as *const CFAllocator }
}

pub type CFIndex = c_long;