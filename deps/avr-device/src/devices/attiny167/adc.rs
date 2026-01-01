#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Miscellaneous Control Register (Shared with CURRENT_SOURCE IO_MODULE)"]
    pub amiscr: AMISCR,
    #[doc = "0x01 - ADC Data Register Bytes"]
    pub adc: ADC,
    #[doc = "0x03 - The ADC Control and Status register A"]
    pub adcsra: ADCSRA,
    #[doc = "0x04 - The ADC Control and Status register B (Shared with ANALOG_COMPARATOR IO_MODULE)"]
    pub adcsrb: ADCSRB,
    #[doc = "0x05 - The ADC multiplexer Selection Register"]
    pub admux: ADMUX,
    _reserved5: [u8; 0x01],
    #[doc = "0x07 - Digital Input Disable Register 0"]
    pub didr0: DIDR0,
    #[doc = "0x08 - Digital Input Disable Register 1"]
    pub didr1: DIDR1,
}
#[doc = "ADC (rw) register accessor: an alias for `Reg<ADC_SPEC>`"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: an alias for `Reg<ADCSRA_SPEC>`"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "The ADC Control and Status register A"]
pub mod adcsra;
#[doc = "ADCSRB (rw) register accessor: an alias for `Reg<ADCSRB_SPEC>`"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "The ADC Control and Status register B (Shared with ANALOG_COMPARATOR IO_MODULE)"]
pub mod adcsrb;
#[doc = "ADMUX (rw) register accessor: an alias for `Reg<ADMUX_SPEC>`"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC multiplexer Selection Register"]
pub mod admux;
#[doc = "AMISCR (rw) register accessor: an alias for `Reg<AMISCR_SPEC>`"]
pub type AMISCR = crate::Reg<amiscr::AMISCR_SPEC>;
#[doc = "Analog Miscellaneous Control Register (Shared with CURRENT_SOURCE IO_MODULE)"]
pub mod amiscr;
#[doc = "DIDR0 (rw) register accessor: an alias for `Reg<DIDR0_SPEC>`"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
#[doc = "DIDR1 (rw) register accessor: an alias for `Reg<DIDR1_SPEC>`"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr1;
