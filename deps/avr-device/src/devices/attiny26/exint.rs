#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Interrupt Flag register"]
    pub gifr: GIFR,
    #[doc = "0x01 - General Interrupt Mask Register"]
    pub gimsk: GIMSK,
}
#[doc = "GIFR (rw) register accessor: an alias for `Reg<GIFR_SPEC>`"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag register"]
pub mod gifr;
#[doc = "GIMSK (rw) register accessor: an alias for `Reg<GIMSK_SPEC>`"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
