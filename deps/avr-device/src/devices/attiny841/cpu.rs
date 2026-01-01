#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose I/O Register 0"]
    pub gpior0: GPIOR0,
    #[doc = "0x01 - General Purpose I/O Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x02 - General Purpose I/O Register 2"]
    pub gpior2: GPIOR2,
    _reserved3: [u8; 0x1e],
    #[doc = "0x21 - MCU Status Register"]
    pub mcusr: MCUSR,
    #[doc = "0x22 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved5: [u8; 0x01],
    #[doc = "0x24 - Store Program Memory Control and Status Register"]
    pub spmcsr: SPMCSR,
    _reserved6: [u8; 0x18],
    #[doc = "0x3d - Power Reduction Register"]
    pub prr: PRR,
    #[doc = "0x3e - Configuration Change Protection"]
    pub ccp: CCP,
    #[doc = "0x3f - Clock Control Register"]
    pub clkcr: CLKCR,
    #[doc = "0x40 - Clock Prescale Register"]
    pub clkpr: CLKPR,
    #[doc = "0x41 - Oscillator Calibration Register 8MHz"]
    pub osccal0: OSCCAL0,
    #[doc = "0x42 - Oscillator Temperature Calibration Register A"]
    pub osctcal0a: OSCTCAL0A,
    #[doc = "0x43 - Oscillator Temperature Calibration Register B"]
    pub osctcal0b: OSCTCAL0B,
    #[doc = "0x44 - Oscillator Calibration Register 32kHz"]
    pub osccal1: OSCCAL1,
}
#[doc = "CCP (rw) register accessor: an alias for `Reg<CCP_SPEC>`"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
#[doc = "CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkcr;
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
#[doc = "OSCCAL0 (rw) register accessor: an alias for `Reg<OSCCAL0_SPEC>`"]
pub type OSCCAL0 = crate::Reg<osccal0::OSCCAL0_SPEC>;
#[doc = "Oscillator Calibration Register 8MHz"]
pub mod osccal0;
#[doc = "OSCCAL1 (rw) register accessor: an alias for `Reg<OSCCAL1_SPEC>`"]
pub type OSCCAL1 = crate::Reg<osccal1::OSCCAL1_SPEC>;
#[doc = "Oscillator Calibration Register 32kHz"]
pub mod osccal1;
#[doc = "OSCTCAL0A (rw) register accessor: an alias for `Reg<OSCTCAL0A_SPEC>`"]
pub type OSCTCAL0A = crate::Reg<osctcal0a::OSCTCAL0A_SPEC>;
#[doc = "Oscillator Temperature Calibration Register A"]
pub mod osctcal0a;
#[doc = "OSCTCAL0B (rw) register accessor: an alias for `Reg<OSCTCAL0B_SPEC>`"]
pub type OSCTCAL0B = crate::Reg<osctcal0b::OSCTCAL0B_SPEC>;
#[doc = "Oscillator Temperature Calibration Register B"]
pub mod osctcal0b;
#[doc = "PRR (rw) register accessor: an alias for `Reg<PRR_SPEC>`"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SPMCSR (rw) register accessor: an alias for `Reg<SPMCSR_SPEC>`"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status Register"]
pub mod spmcsr;
