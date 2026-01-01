#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Pins, Port F"]
    pub pinf: PINF,
    _reserved1: [u8; 0x40],
    #[doc = "0x41 - Data Direction Register, Port F"]
    pub ddrf: DDRF,
    #[doc = "0x42 - Data Register, Port F"]
    pub portf: PORTF,
}
#[doc = "DDRF (rw) register accessor: an alias for `Reg<DDRF_SPEC>`"]
pub type DDRF = crate::Reg<ddrf::DDRF_SPEC>;
#[doc = "Data Direction Register, Port F"]
pub mod ddrf;
#[doc = "PINF (rw) register accessor: an alias for `Reg<PINF_SPEC>`"]
pub type PINF = crate::Reg<pinf::PINF_SPEC>;
#[doc = "Input Pins, Port F"]
pub mod pinf;
#[doc = "PORTF (rw) register accessor: an alias for `Reg<PORTF_SPEC>`"]
pub type PORTF = crate::Reg<portf::PORTF_SPEC>;
#[doc = "Data Register, Port F"]
pub mod portf;
