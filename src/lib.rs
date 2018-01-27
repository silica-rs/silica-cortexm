#![feature(optin_builtin_traits)]
#![feature(core_intrinsics)]
#![feature(integer_atomics)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(asm)]
#![no_std]

use core::ptr;
use core::ops::{BitOr, BitAnd, Not, Deref};
use core::sync::atomic::{AtomicU32, Ordering};

/// Instrumentation Trace Macro Cell
pub mod itm;

/// Private Peripheral Bus
pub mod ppb;

use ppb::scb::get_scb;
/*
#define SCS_BASE            (0xE000E000UL)                            /*!< System Control Space Base Address  */
#define ITM_BASE            (0xE0000000UL)                            /*!< ITM Base Address                   */
#define CoreDebug_BASE      (0xE000EDF0UL)                            /*!< Core Debug Base Address            */
#define SysTick_BASE        (SCS_BASE +  0x0010UL)                    /*!< SysTick Base Address               */
#define NVIC_BASE           (SCS_BASE +  0x0100UL)                    /*!< NVIC Base Address                  */
#define SCB_BASE            (SCS_BASE +  0x0D00UL)                    /*!< System Control Block Base Address  */
*/

#[repr(C)]
pub struct Ro<T>(T);
impl<T> Ro<T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0) }
    }
}
unsafe impl<T> Sync for Ro<T> {}

#[repr(C)]
pub struct Rw<T>(T);
impl<T> Rw<T> where T: BitOr<Output = T> + BitAnd<Output = T> + Not<Output = T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0) }
    }
    pub fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(&mut self.0, value) }
    }
    pub fn update(&mut self, value: T, mask: T) {
        unsafe {
            critical_section_enter();
            let v = self.read() & !mask;
            self.write(v | value);
            critical_section_exit();
        }
    }
}
unsafe impl<T> Sync for Rw<T> {}
impl<T> Deref for Rw<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[repr(C)]
pub struct Wo<T>(T);
impl<T> Wo<T> {
    pub fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(&mut self.0, value) }
    }
}
unsafe impl<T> Sync for Wo<T> {}

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

static CSCNT: AtomicU32 = AtomicU32::new(0);
#[no_mangle]
pub unsafe extern fn critical_section_enter() {
    asm!(  "mrs r0, basepri
            mov r1, $$15
            msr basepri, r1":::"r0","r1");

    let cs = CSCNT.fetch_add(1, Ordering::SeqCst);

    if cs == 0 {
        debug_assert_eq!(get_scb().icsr.read() & 0xFF, 0);
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


pub const SYSTMR_BASE: u32 = 0xE000E010;
pub const NVIC_BASE: u32 = 0xE000E100;
pub const MPU_BASE: u32 = 0xE000ED90;

pub const ISER_BASE: u32 = 0xE000E100;
pub const ICER_BASE: u32 = 0xE000E180;
pub const ISPR_BASE: u32 = 0xE000E200;
pub const ICPR_BASE: u32 = 0xE000E280;
pub const IABR_BASE: u32 = 0xE000E300;
pub const IPR_BASE: u32 = 0xE000E400;
pub const STIR_BASE: u32 = 0xE000EF00;


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
