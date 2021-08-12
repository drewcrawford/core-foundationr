use crate::base::{CFType, CFTypeWithBaseType, CFTypeID, OpaqueCType, CFIndex, CFTypeAny};
#[repr(C)]
pub struct CFArray(OpaqueCType);
impl CFType for CFArray {}
extern "C" {
    fn CFArrayGetTypeID() -> CFTypeID;
    fn CFArrayGetValueAtIndex(theArray: * const CFArray, idx: CFIndex) -> *const CFTypeAny;
    fn CFArrayGetCount(theArray: *const CFArray) -> CFIndex;

}
impl CFTypeWithBaseType for CFArray {
    fn type_id() -> CFTypeID {
        unsafe { CFArrayGetTypeID() }
    }
}
impl CFArray {
    ///# Safety: access beyond the end of the array will throw an exception in objc, which is UB.
    unsafe fn get_unchecked(&self, index: CFIndex) -> &CFTypeAny {
        &*CFArrayGetValueAtIndex(self, index)
    }
    pub fn iter(&self) -> impl Iterator<Item=&CFTypeAny> {
        CFArrayRefIterator {
            array_ref: self,
            current_index: 0,
            last_index: unsafe{ CFArrayGetCount(self)}
        }
    }
}


struct CFArrayRefIterator<'a> {
    array_ref: &'a CFArray,
    current_index: CFIndex,
    last_index: CFIndex
}
impl<'a> Iterator for CFArrayRefIterator<'a> {
    type Item = &'a CFTypeAny;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.last_index {
            None
        }
        else {
            let item = unsafe{ self.array_ref.get_unchecked(self.current_index)};
            self.current_index += 1;
            Some(item)
        }
    }
}





