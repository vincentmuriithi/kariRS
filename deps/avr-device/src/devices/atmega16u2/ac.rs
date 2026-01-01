#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
    _reserved1: [u8; 0x2c],
    #[doc = "0x2d - Analog Comparator Input Multiplexer"]
    pub acmux: ACMUX,
    _reserved2: [u8; 0x01],
    #[doc = "0x2f - No Description."]
    pub didr1: DIDR1,
}
#[doc = "ACMUX (rw) register accessor: an alias for `Reg<ACMUX_SPEC>`"]
pub type ACMUX = crate::Reg<acmux::ACMUX_SPEC>;
#[doc = "Analog Comparator Input Multiplexer"]
pub mod acmux;
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "DIDR1 (rw) register accessor: an alias for `Reg<DIDR1_SPEC>`"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "No Description."]
pub mod didr1;
