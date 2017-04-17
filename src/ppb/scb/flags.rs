/// reserved for debug purpose ! must write 0
pub const AIRCR_VECTRESET: u32 = 0x00000001;
/// reserved for debug purpose ! must write 0
pub const AIRCR_VECTCLRACTIVE: u32 = 0x00000002;
/// asserts a signal to the outer system that requests a reset.
/// (implementation defined, check your chip's documentation)
pub const AIRCR_SYSRESETREQ: u32 = 0x00000004;
pub const AIRCR_PRIGROUP_MASK: u32 = 0x00000700;
pub const AIRCR_PRIGROUP_SHIFT: u32 = 8;
pub const AIRCR_ENDIANNESS_MASK: u32 = 0x00008000;
pub const AIRCR_VECTKEY: u32 = 0xFA050000;
pub const AIRCR_VECTKEY_MASK: u32 = 0xFFFF0000;
