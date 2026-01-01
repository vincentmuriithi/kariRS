#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port F Input Pins Address"]
    pub pinf: PINF,
    #[doc = "0x01 - Port F Data Direction Register"]
    pub ddrf: DDRF,
    #[doc = "0x02 - Port F Data Register"]
    pub portf: PORTF,
}
#[doc = "DDRF (rw) register accessor: an alias for `Reg<DDRF_SPEC>`"]
pub type DDRF = crate::Reg<ddrf::DDRF_SPEC>;
#[doc = "Port F Data Direction Register"]
pub mod ddrf;
#[doc = "PINF (rw) register accessor: an alias for `Reg<PINF_SPEC>`"]
pub type PINF = crate::Reg<pinf::PINF_SPEC>;
#[doc = "Port F Input Pins Address"]
pub mod pinf;
#[doc = "PORTF (rw) register accessor: an alias for `Reg<PORTF_SPEC>`"]
pub type PORTF = crate::Reg<portf::PORTF_SPEC>;
#[doc = "Port F Data Register"]
pub mod portf;
