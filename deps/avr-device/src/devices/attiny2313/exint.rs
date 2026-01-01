#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Extended Interrupt Flag Register"]
    pub eifr: EIFR,
    #[doc = "0x01 - General Interrupt Mask Register"]
    pub gimsk: GIMSK,
}
#[doc = "EIFR (rw) register accessor: an alias for `Reg<EIFR_SPEC>`"]
pub type EIFR = crate::Reg<eifr::EIFR_SPEC>;
#[doc = "Extended Interrupt Flag Register"]
pub mod eifr;
#[doc = "GIMSK (rw) register accessor: an alias for `Reg<GIMSK_SPEC>`"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
