use crate::base::{CFTypeRef, OpaqueCType};

#[repr(C)]
pub struct CFErrorRef(OpaqueCType);
impl CFTypeRef for CFErrorRef {}