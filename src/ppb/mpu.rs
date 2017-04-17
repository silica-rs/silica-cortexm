use ::{Rw, Ro};

#[repr(C)]
pub struct MemoryProtectionUnit {
    pub mpu_type: Ro<u32>,
    pub ctrl: Rw<u32>,
    pub rnr: Rw<u32>,

    rbar: Rw<u32>,
    rasr: Rw<u32>
}

pub enum MemoryType {
    StronglyOrdered,
    Device,
    Normal
}

#[allow(non_camel_case_types)]
pub enum AccessPermission {
    NoAccess_NoAccess,
    Ro_NoAccess,
    Ro_Ro,
    Rw_NoAccess,
    Rw_Ro,
    Rw_Rw
}

const XN_MASK: u32 = 0x10000000;
const AP_MASK: u32 = 0x07000000;
const AP_SHIFT: u32 = 24;
const TEX_MASK: u32 = 0x00380000;
const TEX_SHIFT: u32 = 19;
const S_MASK: u32 = 0x00040000;
const C_MASK: u32 = 0x00020000;
const B_MASK: u32 = 0x00010000;
const SRD_MASK: u32 = 0x0000FF00;
const SRD_SHIFT: u32 = 8;
const EN_MASK: u32 = 0x00000001;

pub struct Region {
    pub base_address: u32,
    pub instruction_access_disabled: bool,
    pub access_permissions: AccessPermission,
    pub type_extension: u8,
    pub cachable: bool,
    pub bufferable: bool,
    pub shareable: bool,
    pub subregion_disable: Option<u8>,
    pub size: u32,
    pub enable: bool
}

impl MemoryProtectionUnit {
    pub fn get_region(&mut self, rnr: u8) -> Region {
        self.rnr.write(rnr as u32);
        let rbar = self.rbar.read();
        let rasr = self.rasr.read();

        let size = (rasr & 0x1E) >> 1;
        let size: u32 = 1 << (size + 1);

        let srd = if (size >= 256) || (size == 0) {
            Some(((rasr & SRD_MASK) >> SRD_SHIFT) as u8)
        } else {
            None
        };

        let ap = match (rasr & AP_MASK) >> AP_SHIFT {
            0b000 => AccessPermission::NoAccess_NoAccess,
            0b101 => AccessPermission::Ro_NoAccess,
            0b110 | 0b111 => AccessPermission::Ro_Ro,
            0b001 => AccessPermission::Rw_NoAccess,
            0b010 => AccessPermission::Rw_Ro,
            0b011 => AccessPermission::Rw_Rw,
            _ => panic!("unspecified value for field AP in MPU->RASR")
        };

        Region {
            base_address: rbar,
            instruction_access_disabled: (rasr & XN_MASK) == XN_MASK,
            access_permissions: ap,
            type_extension: ((rasr & TEX_MASK) >> TEX_SHIFT) as u8,
            cachable: (rasr & C_MASK) == C_MASK,
            bufferable: (rasr & B_MASK) == B_MASK,
            shareable: (rasr & S_MASK) == S_MASK,
            subregion_disable: srd,
            size: size,
            enable: (rasr & EN_MASK) == EN_MASK
        }
    }
    #[allow(unused_variables)]
    pub fn set_region(&mut self, r: &Region) {
        // check size
        // check alignment according to size
        // check tex
        // check srd according to size
        unimplemented!()
    }
}

extern "C" {
    pub static mut MPU: MemoryProtectionUnit;
}
