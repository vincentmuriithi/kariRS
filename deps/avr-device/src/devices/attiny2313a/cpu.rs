#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power reduction register"]
    pub prr: PRR,
    #[doc = "0x01 - BOD control register"]
    pub bodcr: BODCR,
    _reserved2: [u8; 0x0b],
    #[doc = "0x0d - General Purpose I/O Register 0"]
    pub gpior0: GPIOR0,
    #[doc = "0x0e - General Purpose I/O Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x0f - General Purpose I/O Register 2"]
    pub gpior2: GPIOR2,
    _reserved5: [u8; 0x0d],
    #[doc = "0x1d - General Timer Counter Control Register"]
    pub gtccr: GTCCR,
    _reserved6: [u8; 0x02],
    #[doc = "0x20 - Clock Prescale Register"]
    pub clkpr: CLKPR,
    _reserved7: [u8; 0x0a],
    #[doc = "0x2b - Oscillator Calibration Register"]
    pub osccal: OSCCAL,
    _reserved8: [u8; 0x02],
    #[doc = "0x2e - MCU Status register"]
    pub mcusr: MCUSR,
    #[doc = "0x2f - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved10: [u8; 0x01],
    #[doc = "0x31 - Store Program Memory Control and Status register"]
    pub spmcsr: SPMCSR,
    _reserved11: [u8; 0x05],
    #[doc = "0x37 - Stack Pointer Low Byte"]
    pub spl: SPL,
}
#[doc = "BODCR (rw) register accessor: an alias for `Reg<BODCR_SPEC>`"]
pub type BODCR = crate::Reg<bodcr::BODCR_SPEC>;
#[doc = "BOD control register"]
pub mod bodcr;
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
#[doc = "GTCCR (rw) register accessor: an alias for `Reg<GTCCR_SPEC>`"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer Counter Control Register"]
pub mod gtccr;
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
#[doc = "PRR (rw) register accessor: an alias for `Reg<PRR_SPEC>`"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power reduction register"]
pub mod prr;
#[doc = "SPL (rw) register accessor: an alias for `Reg<SPL_SPEC>`"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low Byte"]
pub mod spl;
#[doc = "SPMCSR (rw) register accessor: an alias for `Reg<SPMCSR_SPEC>`"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status register"]
pub mod spmcsr;
