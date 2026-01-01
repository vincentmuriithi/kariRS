#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)"]
    pub amiscr: AMISCR,
}
#[doc = "AMISCR (rw) register accessor: an alias for `Reg<AMISCR_SPEC>`"]
pub type AMISCR = crate::Reg<amiscr::AMISCR_SPEC>;
#[doc = "Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)"]
pub mod amiscr;
