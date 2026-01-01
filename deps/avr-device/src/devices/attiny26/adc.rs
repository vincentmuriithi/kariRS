#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Data Register Bytes"]
    pub adc: ADC,
    #[doc = "0x02 - ADC Control and Status Register"]
    pub adcsr: ADCSR,
    #[doc = "0x03 - The ADC multiplexer Selection Register"]
    pub admux: ADMUX,
}
#[doc = "ADC (rw) register accessor: an alias for `Reg<ADC_SPEC>`"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSR (rw) register accessor: an alias for `Reg<ADCSR_SPEC>`"]
pub type ADCSR = crate::Reg<adcsr::ADCSR_SPEC>;
#[doc = "ADC Control and Status Register"]
pub mod adcsr;
#[doc = "ADMUX (rw) register accessor: an alias for `Reg<ADMUX_SPEC>`"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC multiplexer Selection Register"]
pub mod admux;
