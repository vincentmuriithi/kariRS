#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital Input Disable Register 0"]
    pub didr0: DIDR0,
    #[doc = "0x01 - Digital Input Disable Register 1"]
    pub didr1: DIDR1,
    #[doc = "0x02 - ADC Control and Status Register B"]
    pub adcsrb: ADCSRB,
    #[doc = "0x03 - ADC Data Register Bytes"]
    pub adc: ADC,
    #[doc = "0x05 - The ADC Control and Status register"]
    pub adcsra: ADCSRA,
    #[doc = "0x06 - The ADC multiplexer Selection Register"]
    pub admux: ADMUX,
}
#[doc = "ADC (rw) register accessor: an alias for `Reg<ADC_SPEC>`"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: an alias for `Reg<ADCSRA_SPEC>`"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "The ADC Control and Status register"]
pub mod adcsra;
#[doc = "ADCSRB (rw) register accessor: an alias for `Reg<ADCSRB_SPEC>`"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "ADMUX (rw) register accessor: an alias for `Reg<ADMUX_SPEC>`"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC multiplexer Selection Register"]
pub mod admux;
#[doc = "DIDR0 (rw) register accessor: an alias for `Reg<DIDR0_SPEC>`"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
#[doc = "DIDR1 (rw) register accessor: an alias for `Reg<DIDR1_SPEC>`"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr1;
