use crate::base::{CFType, CFTypeWithBaseType, CFTypeID, OpaqueCType};
#[repr(C)]
pub struct CFArray(OpaqueCType);
impl CFType for CFArray {}
extern "C" {
    fn CFArrayGetTypeID() -> CFTypeID;
}
impl CFTypeWithBaseType for CFArray {
    fn type_id() -> CFTypeID {
        unsafe { CFArrayGetTypeID() }
    }
}

struct CFArrayRefIterator {

}




