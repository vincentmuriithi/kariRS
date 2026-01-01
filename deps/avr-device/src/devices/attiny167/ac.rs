#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
    _reserved1: [u8; 0x2a],
    #[doc = "0x2b - Analog Comparator & ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE)"]
    pub adcsrb: ADCSRB,
}
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ADCSRB (rw) register accessor: an alias for `Reg<ADCSRB_SPEC>`"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "Analog Comparator & ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE)"]
pub mod adcsrb;
