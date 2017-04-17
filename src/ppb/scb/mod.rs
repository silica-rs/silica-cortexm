use ::{Rw, Ro};
use core::intrinsics::unreachable;

mod flags;
pub use self::flags::*;

const ACTLR_BASE: u32 = 0xE000E008;
const SCB_BASE: u32 = 0xE000ED00;

const ACTLR: *mut AuxiliaryControlRegister = ACTLR_BASE as *const AuxiliaryControlRegister as *mut AuxiliaryControlRegister;
const SCB: *mut SystemControlBlock = SCB_BASE as *const SystemControlBlock as *mut SystemControlBlock;

#[repr(C)]
pub struct SystemControlBlock {
    pub cpuid: Ro<u32>,
    pub icsr: Rw<u32>,
    pub vtor: Rw<u32>,
    /// Application Interrupt and Reset Control Register
    pub aircr: Rw<u32>,
    pub scr: Rw<u32>,
    pub ccr: Rw<u32>,
    pub shp: [Rw<u8>; 12],
    pub shcsr: Rw<u32>,
    pub cfsr: Rw<u32>,
    pub hfsr: Rw<u32>,
    pub dfsr: Rw<u32>,
    pub mmfar: Rw<u32>,
    pub bfar: Rw<u32>,
    pub afsr: Rw<u32>,
    pub pfr: [Ro<u32>; 2],
    pub dfr: Ro<u32>,
    pub adr: Ro<u32>,
    pub mmfr: [Ro<u32>; 4],
    pub isar: [Ro<u32>; 5],
    reserved: [u32; 5],
    pub cpacr: Rw<u32>
}

#[repr(C)]
pub struct AuxiliaryControlRegister(Rw<u32>);
impl AuxiliaryControlRegister {
    pub fn read(&self) -> u32 {
        self.0.read()
    }

    pub fn write(&mut self, value: u32) {
        self.0.write(value & 0x7);
    }
}

pub fn get_scb() -> &'static mut SystemControlBlock {
    unsafe { &mut *SCB }
}

pub fn system_reset() -> ! {
    let scb = get_scb();
    scb.aircr.update(AIRCR_VECTKEY | AIRCR_SYSRESETREQ, AIRCR_VECTKEY_MASK | AIRCR_SYSRESETREQ);
    debug_assert!(true, "should not be reached");
    unsafe { unreachable(); }
}
