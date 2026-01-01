#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Reduction Register"]
    pub prr: PRR,
    _reserved1: [u8; 0x12],
    #[doc = "0x13 - General Purpose I/O Register 0"]
    pub gpior0: GPIOR0,
    #[doc = "0x14 - General Purpose I/O Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x15 - General Purpose I/O Register 2"]
    pub gpior2: GPIOR2,
    _reserved4: [u8; 0x10],
    #[doc = "0x26 - Clock Prescale Register"]
    pub clkpr: CLKPR,
    _reserved5: [u8; 0x0a],
    #[doc = "0x31 - Oscillator Calibration Value"]
    pub osccal: OSCCAL,
    _reserved6: [u8; 0x02],
    #[doc = "0x34 - MCU Status Register"]
    pub mcusr: MCUSR,
    #[doc = "0x35 - MCU Control Register"]
    pub mcucr: MCUCR,
}
#[doc = "CLKPR (rw) register accessor: an alias for `Reg<CLKPR_SPEC>`"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "GPIOR0 (rw) register accessor: an alias for `Reg<GPIOR0_SPEC>`"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General Purpose I/O Register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: an alias for `Reg<GPIOR1_SPEC>`"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose I/O Register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: an alias for `Reg<GPIOR2_SPEC>`"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose I/O Register 2"]
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
