#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Store Program Memory Control Register"]
    pub spmcsr: SPMCSR,
}
#[doc = "SPMCSR (rw) register accessor: an alias for `Reg<SPMCSR_SPEC>`"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control Register"]
pub mod spmcsr;
