#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
    _reserved1: [u8; 0x2a],
    #[doc = "0x2b - ADC Control and Status Register B"]
    pub adcsrb: ADCSRB,
    _reserved2: [u8; 0x03],
    #[doc = "0x2f - No Description."]
    pub didr1: DIDR1,
}
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ADCSRB (rw) register accessor: an alias for `Reg<ADCSRB_SPEC>`"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "DIDR1 (rw) register accessor: an alias for `Reg<DIDR1_SPEC>`"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "No Description."]
pub mod didr1;
