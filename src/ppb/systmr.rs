use ::{Rw, Ro};

#[repr(C)]
pub struct SystemTimer {
    pub syst_csr: Rw<u32>,
    pub syst_rvr: Rw<u32>,
    pub syst_cvr: Rw<u32>,
    pub syst_calib: Ro<u32>
}

extern "C" {
    pub static mut SYSTMR: SystemTimer;
}
