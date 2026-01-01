#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital Input Disable Register 1"]
    pub didr: DIDR,
    _reserved1: [u8; 0x06],
    #[doc = "0x07 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
}
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "DIDR (rw) register accessor: an alias for `Reg<DIDR_SPEC>`"]
pub type DIDR = crate::Reg<didr::DIDR_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr;
