#![feature(optin_builtin_traits)]
#![feature(core_intrinsics)]
#![feature(integer_atomics)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(asm)]
#![no_std]

use core::ptr;
use core::ops::{BitOr, BitAnd};
use core::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
pub struct Ro<T>(T);
impl<T> Ro<T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0) }
    }
}

#[repr(C)]
pub struct Rw<T>(T);
impl<T> Rw<T> where T: BitOr<Output = T> + BitAnd<Output = T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0) }
    }
    pub fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(&mut self.0, value) }
    }
    pub fn raise_flags(&mut self, mask: T) {
        let v = self.read() | mask;
        self.write(v);
    }
    pub fn clear_flags(&mut self, mask: T) {
        let v = self.read() & mask;
        self.write(v);
    }
}
impl<T> !Sync for Rw<T> {}

#[repr(C)]
pub struct Wo<T>(T);
impl<T> Wo<T> {
    pub fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(&mut self.0, value) }
    }
}

pub type Handler = unsafe extern "C" fn ();

#[repr(C)]
pub struct Exceptions {
    pub reset: unsafe extern "C" fn () -> !,
    pub nmi: Handler,
    pub hard_fault: Handler,
    pub mem_manage: Handler,
    pub bus_fault: Handler,
    pub usage_fault: Handler,
    pub reserved1: [u32; 4],
    pub sv_call: Handler,
    pub debug_monitor: Handler,
    pub reserved2: u32,
    pub pendsv: Handler,
    pub systick:  Handler
}

pub mod ppb;

static CSCNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern fn critical_section_enter() {
    asm!(  "mrs r0, basepri
            mov r1, $$15
            msr basepri, r1":::"r0","r1");

    let cs = CSCNT.fetch_add(1, Ordering::SeqCst);

    if cs == 0 {
        debug_assert_eq!(ppb::scb::SCB.icsr.read() & 0xFF, 0);
    }
}

#[no_mangle]
pub unsafe extern fn critical_section_exit() {
    let cs = CSCNT.fetch_sub(1, Ordering::SeqCst);
    debug_assert!(cs != 0);

    if cs == 1 {
        asm!("msr basepri, $0"::"r"(0));
    }
}

/*
#[cfg(target_arch = "arm")]
unsafe extern "C" fn default_handler() -> ! {
    let current_isr: u32;
    asm!("msr $2, ipsr": "=r"(current_isr));

    if let Some(handler) = isr_handler {
        handler();
    }
}
*/
