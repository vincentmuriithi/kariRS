#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Store Program Memory Control Register"]
    pub spmcr: SPMCR,
}
#[doc = "SPMCR (rw) register accessor: an alias for `Reg<SPMCR_SPEC>`"]
pub type SPMCR = crate::Reg<spmcr::SPMCR_SPEC>;
#[doc = "Store Program Memory Control Register"]
pub mod spmcr;
