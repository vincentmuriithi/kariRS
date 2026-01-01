#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PORT H Input Pins"]
    pub pinh: PINH,
    #[doc = "0x01 - PORT H Data Direction Register"]
    pub ddrh: DDRH,
    #[doc = "0x02 - PORT H Data Register"]
    pub porth: PORTH,
}
#[doc = "DDRH (rw) register accessor: an alias for `Reg<DDRH_SPEC>`"]
pub type DDRH = crate::Reg<ddrh::DDRH_SPEC>;
#[doc = "PORT H Data Direction Register"]
pub mod ddrh;
#[doc = "PINH (rw) register accessor: an alias for `Reg<PINH_SPEC>`"]
pub type PINH = crate::Reg<pinh::PINH_SPEC>;
#[doc = "PORT H Input Pins"]
pub mod pinh;
#[doc = "PORTH (rw) register accessor: an alias for `Reg<PORTH_SPEC>`"]
pub type PORTH = crate::Reg<porth::PORTH_SPEC>;
#[doc = "PORT H Data Register"]
pub mod porth;
