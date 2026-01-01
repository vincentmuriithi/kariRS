#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub portcr: PORTCR,
    _reserved1: [u8; 0x0b],
    #[doc = "0x0c - General purpose register 0"]
    pub gpior0: GPIOR0,
    _reserved2: [u8; 0x0b],
    #[doc = "0x18 - General Purpose register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x19 - General Purpose IO register 2"]
    pub gpior2: GPIOR2,
    _reserved4: [u8; 0x05],
    #[doc = "0x1f - DebugWire data register"]
    pub dwdr: DWDR,
    _reserved5: [u8; 0x01],
    #[doc = "0x21 - Sleep Mode Control Register"]
    pub smcr: SMCR,
    #[doc = "0x22 - MCU Status register"]
    pub mcusr: MCUSR,
    #[doc = "0x23 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved8: [u8; 0x0b],
    #[doc = "0x2f - Clock Prescale Register"]
    pub clkpr: CLKPR,
    #[doc = "0x30 - Clock Control & Status Register"]
    pub clkcsr: CLKCSR,
    #[doc = "0x31 - Clock Selection Register"]
    pub clkselr: CLKSELR,
    #[doc = "0x32 - Power Reduction Register"]
    pub prr: PRR,
    _reserved12: [u8; 0x01],
    #[doc = "0x34 - Oscillator Calibration Register"]
    pub osccal: OSCCAL,
}
#[doc = "CLKCSR (rw) register accessor: an alias for `Reg<CLKCSR_SPEC>`"]
pub type CLKCSR = crate::Reg<clkcsr::CLKCSR_SPEC>;
#[doc = "Clock Control & Status Register"]
pub mod clkcsr;
#[doc = "CLKPR (rw) register accessor: an alias for `Reg<CLKPR_SPEC>`"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "CLKSELR (rw) register accessor: an alias for `Reg<CLKSELR_SPEC>`"]
pub type CLKSELR = crate::Reg<clkselr::CLKSELR_SPEC>;
#[doc = "Clock Selection Register"]
pub mod clkselr;
#[doc = "DWDR (rw) register accessor: an alias for `Reg<DWDR_SPEC>`"]
pub type DWDR = crate::Reg<dwdr::DWDR_SPEC>;
#[doc = "DebugWire data register"]
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
#[doc = "PORTCR (rw) register accessor: an alias for `Reg<PORTCR_SPEC>`"]
pub type PORTCR = crate::Reg<portcr::PORTCR_SPEC>;
#[doc = "Port Control Register"]
pub mod portcr;
#[doc = "PRR (rw) register accessor: an alias for `Reg<PRR_SPEC>`"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "Sleep Mode Control Register"]
pub mod smcr;
