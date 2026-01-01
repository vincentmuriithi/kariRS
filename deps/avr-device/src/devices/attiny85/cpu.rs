#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General purpose register 0"]
    pub gpior0: GPIOR0,
    #[doc = "0x01 - General Purpose register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x02 - General Purpose IO register 2"]
    pub gpior2: GPIOR2,
    _reserved3: [u8; 0x0c],
    #[doc = "0x0f - Power Reduction Register"]
    pub prr: PRR,
    _reserved4: [u8; 0x01],
    #[doc = "0x11 - debugWire data register"]
    pub dwdr: DWDR,
    _reserved5: [u8; 0x03],
    #[doc = "0x15 - Clock Prescale Register"]
    pub clkpr: CLKPR,
    #[doc = "0x16 - PLL Control and status register"]
    pub pllcsr: PLLCSR,
    _reserved7: [u8; 0x09],
    #[doc = "0x20 - Oscillator Calibration Register"]
    pub osccal: OSCCAL,
    _reserved8: [u8; 0x02],
    #[doc = "0x23 - MCU Status register"]
    pub mcusr: MCUSR,
    #[doc = "0x24 - MCU Control Register"]
    pub mcucr: MCUCR,
}
#[doc = "CLKPR (rw) register accessor: an alias for `Reg<CLKPR_SPEC>`"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "DWDR (rw) register accessor: an alias for `Reg<DWDR_SPEC>`"]
pub type DWDR = crate::Reg<dwdr::DWDR_SPEC>;
#[doc = "debugWire data register"]
pub mod dwdr;
#[doc = "GPIOR0 (rw) register accessor: an alias for `Reg<GPIOR0_SPEC>`"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General purpose register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: an alias for `Reg<GPIOR1_SPEC>`"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: an alias for `Reg<GPIOR2_SPEC>`"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose IO register 2"]
pub mod gpior2;
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUSR (rw) register accessor: an alias for `Reg<MCUSR_SPEC>`"]
pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
#[doc = "MCU Status register"]
pub mod mcusr;
#[doc = "OSCCAL (rw) register accessor: an alias for `Reg<OSCCAL_SPEC>`"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Register"]
pub mod osccal;
#[doc = "PLLCSR (rw) register accessor: an alias for `Reg<PLLCSR_SPEC>`"]
pub type PLLCSR = crate::Reg<pllcsr::PLLCSR_SPEC>;
#[doc = "PLL Control and status register"]
pub mod pllcsr;
#[doc = "PRR (rw) register accessor: an alias for `Reg<PRR_SPEC>`"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
