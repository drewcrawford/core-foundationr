use crate::base::{CFType, CFAllocator, CFIndex, OpaqueCType};
use crate::cell::StrongCell;

extern "C" {
    fn CFDataCreate(allocator: *const CFAllocator, bytes: *const u8, length: CFIndex) -> *const CFData;
    fn CFDataGetLength(theData: *const CFData) -> CFIndex;
    fn CFDataGetBytePtr(theData: *const CFData) -> *const u8;
}

#[repr(C)]
pub struct CFData(OpaqueCType);
impl CFType for CFData {}
#[allow(non_snake_case)]
impl CFData {
    ///- note: objc knows a faster path for owned strings
    ///- note: uncertain about faster path for static strings?
    pub fn from_str(str: &str) -> StrongCell<CFData> {
        let raw = unsafe{ CFDataCreate(CFAllocator::null(), str.as_ptr(), str.as_bytes().len() as CFIndex) };
        unsafe{ StrongCell::assuming_retained_nonnull(raw) }
    }
    ///Copies the provided slice into the CFData.
    pub fn copy_slice(slice: &[u8]) -> StrongCell<CFData> {
        let raw = unsafe { CFDataCreate(CFAllocator::null(), slice.as_ptr(), slice.len().try_into().unwrap()) };
        unsafe { StrongCell::assuming_retained_nonnull(raw) }
    }
    pub fn GetLength(&self) -> CFIndex {
        unsafe { CFDataGetLength(self) }
    }
    pub fn GetBytePtr(&self) -> *const u8 {
        unsafe {
            CFDataGetBytePtr(self)
        }
    }
    pub fn as_slice(&self) -> &[u8] {
        let length = self.GetLength();
        unsafe {
            std::slice::from_raw_parts(self.GetBytePtr(), length as usize)
        }
    }
}

#[test] fn from_str() {
    use crate::CFTypeBehavior;
    let result = CFData::from_str("hello world");
    let str = result.description().as_string();
    assert!(str.starts_with("<CFData"))
}

#[test] fn copy_slice() {
    use crate::CFTypeBehavior;
    let result = CFData::copy_slice(b"hello world");
    let str = result.description().as_string();
    assert!(str.starts_with("<CFData"))
}