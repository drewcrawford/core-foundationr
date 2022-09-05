use std::fmt::{Debug, Display};
use crate::base::{CFString, CFAllocator, CFIndex, CFRange};
use crate::cell::StrongCell;

#[repr(transparent)]
#[derive(Debug)]
pub struct CFStringEncoding(u32);

#[allow(non_upper_case_globals)]
impl CFStringEncoding {
    pub const MacRoman: CFStringEncoding = CFStringEncoding(0);
    pub const WindowsLatin1: CFStringEncoding = CFStringEncoding(0x0500);
    pub const ISOLatin1: CFStringEncoding = CFStringEncoding(0x0201);
    pub const NextStepLatin: CFStringEncoding = CFStringEncoding(0x0B01);
    pub const ASCII: CFStringEncoding = CFStringEncoding(0x0600);
    pub const Unicode: CFStringEncoding = CFStringEncoding(0x0100);
    pub const UTF8: CFStringEncoding = CFStringEncoding(0x08000100);
    pub const NonLossyASCII: CFStringEncoding = CFStringEncoding(0x0BFF);
    pub const UTF16: CFStringEncoding = CFStringEncoding(0x0100);
    pub const UTF16BE: CFStringEncoding = CFStringEncoding(0x10000100);
    pub const UTF16LE: CFStringEncoding = CFStringEncoding(0x14000100);
    pub const UTF32: CFStringEncoding = CFStringEncoding(0x0c000100);
    pub const UTF32BE: CFStringEncoding = CFStringEncoding(0x18000100);
    pub const UTF32LE: CFStringEncoding = CFStringEncoding(0x1c000100);
}

impl CFString {
    ///note: objc knows a faster way for static strings
    ///note: objc knows a faster way for owned strings
    pub fn from_str(str: &str) -> StrongCell<CFString> {
        unsafe{
            let raw = CFStringCreateWithBytes(CFAllocator::null(), str.as_ptr(), str.as_bytes().len() as CFIndex, CFStringEncoding::UTF8, false);
            StrongCell::assuming_retained_nonnull(raw)
        }
    }
    ///Returns the length of the string, see cocoa docs for `CFStringGetLength`.
    pub fn length(&self) -> CFIndex {
        unsafe { CFStringGetLength(self)}
    }
    ///note: objc knows a faster way to return an inner pointer in some cases
    pub fn as_string(&self) -> String {
        let length = self.length();
        let requested_capacity = unsafe{ CFStringGetMaximumSizeForEncoding(length, CFStringEncoding::UTF8) };
        let mut string = String::with_capacity(requested_capacity as usize);
        let actual_capaicty = string.capacity();
        let mut_ptr = string.as_mut_ptr();
        let range = CFRange {
            location: 0,
            length
        };
        let mut used_buf_len = 0;
        unsafe{ CFStringGetBytes(self, range, CFStringEncoding::UTF8, 255, false, mut_ptr, string.capacity() as i64, &mut used_buf_len)};
        std::mem::forget(string);
        unsafe{ String::from_raw_parts(mut_ptr, used_buf_len as usize, actual_capaicty)}
    }
}
impl Debug for CFString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_string())
    }
}
impl Display for CFString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_string())
    }
}


#[link(name="CoreFoundation",kind="framework")]
extern "C" {
    fn CFStringCreateWithBytes(alloc: *const CFAllocator, bytes: *const u8, numBytes: CFIndex, encoding: CFStringEncoding, isExternalRepresentation: bool ) -> *const CFString;
    fn CFStringGetBytes(theString: *const CFString, range: CFRange, encoding: CFStringEncoding, lossByte: u8, isExternalRepresentation: bool, buffer: *mut u8, maxBufferLen: CFIndex, usedBufLen: *mut CFIndex) -> CFIndex;
    fn CFStringGetLength(theString: *const CFString) -> CFIndex;
    fn CFStringGetMaximumSizeForEncoding(length: CFIndex, encoding: CFStringEncoding) -> CFIndex;

}

#[test] fn create_string() {

    let string = CFString::from_str("test");
    println!("cf {:?}",string);
    println!("roundtrip {:?}",string.as_string());

}