#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - XOSC Failure Detection Control and Status Register"]
    pub xfdcsr: XFDCSR,
}
#[doc = "XFDCSR (rw) register accessor: an alias for `Reg<XFDCSR_SPEC>`"]
pub type XFDCSR = crate::Reg<xfdcsr::XFDCSR_SPEC>;
#[doc = "XOSC Failure Detection Control and Status Register"]
pub mod xfdcsr;
