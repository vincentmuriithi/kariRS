#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Flags"]
    pub rstfr: RSTFR,
    #[doc = "0x01 - Software Reset"]
    pub swrr: SWRR,
}
#[doc = "RSTFR (rw) register accessor: an alias for `Reg<RSTFR_SPEC>`"]
pub type RSTFR = crate::Reg<rstfr::RSTFR_SPEC>;
#[doc = "Reset Flags"]
pub mod rstfr;
#[doc = "SWRR (rw) register accessor: an alias for `Reg<SWRR_SPEC>`"]
pub type SWRR = crate::Reg<swrr::SWRR_SPEC>;
#[doc = "Software Reset"]
pub mod swrr;
