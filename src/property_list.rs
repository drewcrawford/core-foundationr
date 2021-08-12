use crate::base::{CFType, CFOptionFlags, CFAllocator, CFIndex, OpaqueCType};
use crate::data::CFData;
use crate::error::CFError;
use crate::cell::StrongCell;
use crate::CFTypeBehavior;
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

#[repr(C)]
pub struct CFPropertyList(OpaqueCType);
impl CFType for CFPropertyList {}

impl CFPropertyList {
    pub fn from_data(data: &CFData) -> Result<StrongCell<CFPropertyList>,*const CFError> {
        let mut err = unsafe{ CFError::from_ptr(std::ptr::null())};
        let o = unsafe{ CFPropertyListCreateWithData(CFAllocator::null(), data, MutabilityOptions::Immutable, std::ptr::null_mut(), &mut err)};
        if !err.is_null() {
            Err(err)
        }
        else {
            Ok(unsafe{ StrongCell::assuming_retained(o)})
        }
    }
}

extern "C" {
    fn CFPropertyListCreateWithData(allocator: *const CFAllocator, data: *const CFData, options: MutabilityOptions, format: *mut Format, error: *mut *const CFError) -> *const CFPropertyList;

}

#[test] fn parse() {
    use crate::{CFTypeBehavior,CFDictionary,CFString,CFArray};

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
    let data = CFData::from_str(str);
    let property_list = CFPropertyList::from_data(&data).unwrap();
    println!("Parsed list {:?}",property_list.description().as_string());
    let dictionary: StrongCell<CFDictionary> = property_list.cast_checked();
    println!("Dictionary {:?}",dictionary);
    let strong_str = CFString::from_str("system-entities");
    let borrow: &CFString = &strong_str;
    let system_entities = dictionary.get_with_key(borrow);
    let array: &CFArray = system_entities.unwrap().checked_cast();
    println!("Array {:?}",array.description().as_string());

    let r = array.iter().find_map(|p| {
        let d: &CFDictionary = p.checked_cast();
        d.get_with_key(&*CFString::from_str("mount-point"))
    });
    let str_mount_point: &CFString = r.unwrap().checked_cast();
    assert_eq!(str_mount_point.as_string(), "/Volumes/macOS Developer Beta Access Utility");
}