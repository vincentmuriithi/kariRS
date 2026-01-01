#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose IO Register 0"]
    pub gpior0: GPIOR0,
    _reserved1: [u8; 0x0b],
    #[doc = "0x0c - General Purpose IO Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x0d - General Purpose IO Register 2"]
    pub gpior2: GPIOR2,
    _reserved3: [u8; 0x07],
    #[doc = "0x15 - Sleep Mode Control Register"]
    pub smcr: SMCR,
    #[doc = "0x16 - MCU Status Register"]
    pub mcusr: MCUSR,
    #[doc = "0x17 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved6: [u8; 0x0b],
    #[doc = "0x23 - No Description."]
    pub clkpr: CLKPR,
    _reserved7: [u8; 0x02],
    #[doc = "0x26 - Power Reduction Register"]
    pub prr: PRR,
    _reserved8: [u8; 0x01],
    #[doc = "0x28 - Oscillator Calibration Value"]
    pub osccal: OSCCAL,
}
#[doc = "CLKPR (rw) register accessor: an alias for `Reg<CLKPR_SPEC>`"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "No Description."]
pub mod clkpr;
#[doc = "GPIOR0 (rw) register accessor: an alias for `Reg<GPIOR0_SPEC>`"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General Purpose IO Register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: an alias for `Reg<GPIOR1_SPEC>`"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose IO Register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: an alias for `Reg<GPIOR2_SPEC>`"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose IO Register 2"]
pub mod gpior2;
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUSR (rw) register accessor: an alias for `Reg<MCUSR_SPEC>`"]
pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
#[doc = "MCU Status Register"]
pub mod mcusr;
#[doc = "OSCCAL (rw) register accessor: an alias for `Reg<OSCCAL_SPEC>`"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Value"]
pub mod osccal;
#[doc = "PRR (rw) register accessor: an alias for `Reg<PRR_SPEC>`"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "Sleep Mode Control Register"]
pub mod smcr;
