#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Reduction Register"]
    pub prr: PRR,
    #[doc = "0x01 - Clock Prescale Register"]
    pub clkpr: CLKPR,
    _reserved2: [u8; 0x07],
    #[doc = "0x09 - Debug Wire Data Register"]
    pub dwdr: DWDR,
    _reserved3: [u8; 0x01],
    #[doc = "0x0b - BOD Control Register"]
    pub bodcr: BODCR,
    #[doc = "0x0c - Oscillator Calibration Register"]
    pub osccal: OSCCAL,
    _reserved5: [u8; 0x02],
    #[doc = "0x0f - MCU Status register"]
    pub mcusr: MCUSR,
    #[doc = "0x10 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved7: [u8; 0x01],
    #[doc = "0x12 - Store Program Memory Control and Status Register"]
    pub spmcsr: SPMCSR,
    _reserved8: [u8; 0x05],
    #[doc = "0x18 - Stack Pointer Low Byte"]
    pub spl: SPL,
}
#[doc = "BODCR (rw) register accessor: an alias for `Reg<BODCR_SPEC>`"]
pub type BODCR = crate::Reg<bodcr::BODCR_SPEC>;
#[doc = "BOD Control Register"]
pub mod bodcr;
#[doc = "CLKPR (rw) register accessor: an alias for `Reg<CLKPR_SPEC>`"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "DWDR (rw) register accessor: an alias for `Reg<DWDR_SPEC>`"]
pub type DWDR = crate::Reg<dwdr::DWDR_SPEC>;
#[doc = "Debug Wire Data Register"]
pub mod dwdr;
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
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SPL (rw) register accessor: an alias for `Reg<SPL_SPEC>`"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low Byte"]
pub mod spl;
#[doc = "SPMCSR (rw) register accessor: an alias for `Reg<SPMCSR_SPEC>`"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status Register"]
pub mod spmcsr;
