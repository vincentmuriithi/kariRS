#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Data Register Bytes"]
    pub adc: ADC,
    #[doc = "0x02 - The ADC Control and Status register"]
    pub adcsra: ADCSRA,
    #[doc = "0x03 - The ADC multiplexer Selection Register"]
    pub admux: ADMUX,
    _reserved3: [u8; 0x28],
    #[doc = "0x2c - Special Function IO Register"]
    pub sfior: SFIOR,
}
#[doc = "ADC (rw) register accessor: an alias for `Reg<ADC_SPEC>`"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: an alias for `Reg<ADCSRA_SPEC>`"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "The ADC Control and Status register"]
pub mod adcsra;
#[doc = "ADMUX (rw) register accessor: an alias for `Reg<ADMUX_SPEC>`"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC multiplexer Selection Register"]
pub mod admux;
#[doc = "SFIOR (rw) register accessor: an alias for `Reg<SFIOR_SPEC>`"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
