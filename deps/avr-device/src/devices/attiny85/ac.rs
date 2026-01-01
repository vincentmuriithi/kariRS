#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control and Status Register B"]
    pub adcsrb: ADCSRB,
    _reserved1: [u8; 0x04],
    #[doc = "0x05 - Analog Comparator Control And Status Register"]
    pub acsr: ACSR,
    _reserved2: [u8; 0x0b],
    #[doc = "0x11 - Digital Input Disable Register 0"]
    pub didr0: DIDR0,
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
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
