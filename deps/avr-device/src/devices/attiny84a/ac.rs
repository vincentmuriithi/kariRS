#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital Input Disable Register"]
    pub didr0: DIDR0,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - ADC Control and Status Register B"]
    pub adcsrb: ADCSRB,
    _reserved2: [u8; 0x04],
    #[doc = "0x07 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
}
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ADCSRB (rw) register accessor: an alias for `Reg<ADCSRB_SPEC>`"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "DIDR0 (rw) register accessor: an alias for `Reg<DIDR0_SPEC>`"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register"]
pub mod didr0;
