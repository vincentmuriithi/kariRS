#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control and Status Register B"]
    pub adcsrb: ADCSRB,
    #[doc = "0x01 - The ADC Control and Status register"]
    pub adcsra: ADCSRA,
    #[doc = "0x02 - ADC Data Register Bytes"]
    pub adc: ADC,
    #[doc = "0x04 - The ADC multiplexer Selection Register B"]
    pub admuxb: ADMUXB,
    #[doc = "0x05 - The ADC multiplexer Selection Register A"]
    pub admuxa: ADMUXA,
    _reserved5: [u8; 0x36],
    #[doc = "0x3c - Digital Input Disable Register 0"]
    pub didr0: DIDR0,
    #[doc = "0x3d - Digital Input Disable Register 1"]
    pub didr1: DIDR1,
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
#[doc = "ADMUXA (rw) register accessor: an alias for `Reg<ADMUXA_SPEC>`"]
pub type ADMUXA = crate::Reg<admuxa::ADMUXA_SPEC>;
#[doc = "The ADC multiplexer Selection Register A"]
pub mod admuxa;
#[doc = "ADMUXB (rw) register accessor: an alias for `Reg<ADMUXB_SPEC>`"]
pub type ADMUXB = crate::Reg<admuxb::ADMUXB_SPEC>;
#[doc = "The ADC multiplexer Selection Register B"]
pub mod admuxb;
#[doc = "DIDR0 (rw) register accessor: an alias for `Reg<DIDR0_SPEC>`"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
#[doc = "DIDR1 (rw) register accessor: an alias for `Reg<DIDR1_SPEC>`"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr1;
