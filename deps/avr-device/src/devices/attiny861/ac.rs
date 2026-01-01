#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register A"]
    pub acsra: ACSRA,
    #[doc = "0x01 - Analog Comparator Control And Status Register B"]
    pub acsrb: ACSRB,
}
#[doc = "ACSRA (rw) register accessor: an alias for `Reg<ACSRA_SPEC>`"]
pub type ACSRA = crate::Reg<acsra::ACSRA_SPEC>;
#[doc = "Analog Comparator Control And Status Register A"]
pub mod acsra;
#[doc = "ACSRB (rw) register accessor: an alias for `Reg<ACSRB_SPEC>`"]
pub type ACSRB = crate::Reg<acsrb::ACSRB_SPEC>;
#[doc = "Analog Comparator Control And Status Register B"]
pub mod acsrb;
