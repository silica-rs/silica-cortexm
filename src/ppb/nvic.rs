use ::{Rw, Wo};

extern "C" {
    pub static ISER: [Rw<u32>; 8];
    pub static ICER: [Rw<u32>; 8];
    pub static ISPR: [Rw<u32>; 8];
    pub static ICPR: [Rw<u32>; 8];
    pub static IABR: [Rw<u32>; 8];
    pub static IPR: [Rw<u32>; 60];
    pub static STIR: Wo<u32>;
}
