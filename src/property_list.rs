use std::ffi::c_void;
use crate::base::{CFTypeRef, CFOptionFlags, CFAllocatorRef, CFIndex};
use crate::data::CFDataRef;
use crate::error::CFErrorRef;
use crate::cell::StrongCell;
use crate::dictionary::CFDictionaryRef;
use crate::CFStringRef;
use crate::array::CFArrayRef;

#[repr(transparent)]
pub struct MutabilityOptions(CFOptionFlags);
#[allow(non_upper_case_globals)]
impl MutabilityOptions {
    pub const Immutable: MutabilityOptions = MutabilityOptions(0);
    pub const MutableContainers: MutabilityOptions = MutabilityOptions(1 << 0);
    pub const MutableContainersAndLeaves: MutabilityOptions = MutabilityOptions(1 << 1);
}

#[repr(transparent)]
pub struct Format(CFIndex);
#[allow(non_upper_case_globals)]
impl Format {
    pub const OpenStepFormat: Format = Format(1);
    pub const XMLFormat_v1_0: Format =  Format(100);
    pub const BinaryFormat_v1_0: Format = Format(200);
}

#[repr(transparent)]
#[derive(Debug,Clone)]
pub struct CFPropertyListRef(*const c_void);
impl CFTypeRef for CFPropertyListRef {
    fn as_ptr(&self) -> *const c_void {
        self.0
    }
    unsafe fn from_ptr(ptr: *const c_void) -> Self { Self(ptr)}
}

impl CFPropertyListRef {
    pub fn from_data(data: CFDataRef) -> Result<StrongCell<CFPropertyListRef>,CFErrorRef> {
        let mut err = unsafe{ CFErrorRef::from_ptr(std::ptr::null())};
        let o = unsafe{ CFPropertyListCreateWithData(CFAllocatorRef::null(), data, MutabilityOptions::Immutable, std::ptr::null_mut(), &mut err)};
        if !err.as_ptr().is_null() {
            Err(err)
        }
        else {
            Ok(unsafe{ StrongCell::assuming_retained(o)})
        }
    }
}

extern "C" {
    fn CFPropertyListCreateWithData(allocator: CFAllocatorRef, data: CFDataRef , options: MutabilityOptions, format: *mut Format, error: *mut CFErrorRef) -> CFPropertyListRef;

}

#[test] fn parse() {
    use crate::prelude::*;
    let str = r#"<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>system-entities</key>
	<array>
		<dict>
			<key>content-hint</key>
			<string>Apple_partition_scheme</string>
			<key>dev-entry</key>
			<string>/dev/disk4</string>
			<key>potentially-mountable</key>
			<false/>
			<key>unmapped-content-hint</key>
			<string>Apple_partition_scheme</string>
		</dict>
		<dict>
			<key>content-hint</key>
			<string>Apple_partition_map</string>
			<key>dev-entry</key>
			<string>/dev/disk4s1</string>
			<key>potentially-mountable</key>
			<false/>
			<key>unmapped-content-hint</key>
			<string>Apple_partition_map</string>
		</dict>
		<dict>
			<key>content-hint</key>
			<string>Apple_HFS</string>
			<key>dev-entry</key>
			<string>/dev/disk4s2</string>
			<key>mount-point</key>
			<string>/Volumes/macOS Developer Beta Access Utility</string>
			<key>potentially-mountable</key>
			<true/>
			<key>unmapped-content-hint</key>
			<string>Apple_HFS</string>
			<key>volume-kind</key>
			<string>hfs</string>
		</dict>
	</array>
</dict>
</plist>"#;
    let data = CFDataRef::from_str(str);
    let property_list = CFPropertyListRef::from_data((*data).clone()).unwrap();
    println!("Parsed list {:?}",property_list.description().as_string());
    let dictionary: StrongCell<CFDictionaryRef> = unsafe{ property_list.cast_checked() };
    println!("Dictionary {:?}",dictionary);
    let strong_str = CFStringRef::from_str("system-entities");
    let system_entities = unsafe{ dictionary.get_with_key((*strong_str).clone())};
    let array: CFArrayRef = system_entities.checked_cast();
    println!("Array {:?}",array.description().as_string());
}