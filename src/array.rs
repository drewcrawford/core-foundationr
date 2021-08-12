use crate::base::{CFTypeRef, CFTypeRefWithBaseType, CFTypeID, OpaqueCType};
#[repr(C)]
pub struct CFArrayRef(OpaqueCType);
impl CFTypeRef for CFArrayRef {}
extern "C" {
    fn CFArrayGetTypeID() -> CFTypeID;
}
impl CFTypeRefWithBaseType for CFArrayRef {
    fn type_id() -> CFTypeID {
        unsafe { CFArrayGetTypeID() }
    }
}

struct CFArrayRefIterator {

}




