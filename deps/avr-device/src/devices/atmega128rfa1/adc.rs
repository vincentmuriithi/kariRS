#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The ADC Control and Status Register C"]
    pub adcsrc: ADCSRC,
    #[doc = "0x01 - ADC Data Register Bytes"]
    pub adc: ADC,
    #[doc = "0x03 - The ADC Control and Status Register A"]
    pub adcsra: ADCSRA,
    #[doc = "0x04 - The ADC Control and Status Register B"]
    pub adcsrb: ADCSRB,
    #[doc = "0x05 - The ADC Multiplexer Selection Register"]
    pub admux: ADMUX,
    #[doc = "0x06 - Digital Input Disable Register 2"]
    pub didr2: DIDR2,
    #[doc = "0x07 - Digital Input Disable Register 0"]
    pub didr0: DIDR0,
}
#[doc = "ADC (rw) register accessor: an alias for `Reg<ADC_SPEC>`"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: an alias for `Reg<ADCSRA_SPEC>`"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "The ADC Control and Status Register A"]
pub mod adcsra;
#[doc = "ADCSRB (rw) register accessor: an alias for `Reg<ADCSRB_SPEC>`"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "The ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "ADCSRC (rw) register accessor: an alias for `Reg<ADCSRC_SPEC>`"]
pub type ADCSRC = crate::Reg<adcsrc::ADCSRC_SPEC>;
#[doc = "The ADC Control and Status Register C"]
pub mod adcsrc;
#[doc = "ADMUX (rw) register accessor: an alias for `Reg<ADMUX_SPEC>`"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC Multiplexer Selection Register"]
pub mod admux;
#[doc = "DIDR0 (rw) register accessor: an alias for `Reg<DIDR0_SPEC>`"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
#[doc = "DIDR2 (rw) register accessor: an alias for `Reg<DIDR2_SPEC>`"]
pub type DIDR2 = crate::Reg<didr2::DIDR2_SPEC>;
#[doc = "Digital Input Disable Register 2"]
pub mod didr2;
