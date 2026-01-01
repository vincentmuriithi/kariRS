#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC0 Reference"]
    pub acref: ACREF,
}
#[doc = "ACREF (rw) register accessor: an alias for `Reg<ACREF_SPEC>`"]
pub type ACREF = crate::Reg<acref::ACREF_SPEC>;
#[doc = "ADC0 Reference"]
pub mod acref;
