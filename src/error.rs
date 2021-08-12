use crate::base::{CFType, OpaqueCType};

#[repr(C)]
pub struct CFError(OpaqueCType);
impl CFType for CFError {}