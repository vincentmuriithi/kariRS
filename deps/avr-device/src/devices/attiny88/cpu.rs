#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Configuration Register"]
    pub portcr: PORTCR,
    _reserved1: [u8; 0x0b],
    #[doc = "0x0c - General Purpose I/O Register 0"]
    pub gpior0: GPIOR0,
    _reserved2: [u8; 0x0b],
    #[doc = "0x18 - General Purpose I/O Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x19 - General Purpose I/O Register 2"]
    pub gpior2: GPIOR2,
    _reserved4: [u8; 0x07],
    #[doc = "0x21 - Sleep Mode Control Register"]
    pub smcr: SMCR,
    #[doc = "0x22 - MCU Status Register"]
    pub mcusr: MCUSR,
    #[doc = "0x23 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved7: [u8; 0x01],
    #[doc = "0x25 - Store Program Memory Control Register"]
    pub spmcsr: SPMCSR,
    _reserved8: [u8; 0x05],
    #[doc = "0x2b - Stack Pointer Low"]
    pub spl: SPL,
    #[doc = "0x2c - Stack Pointer High"]
    pub sph: SPH,
    _reserved10: [u8; 0x02],
    #[doc = "0x2f - Clock Prescale Register"]
    pub clkpr: CLKPR,
    _reserved11: [u8; 0x02],
    #[doc = "0x32 - Power Reduction Register"]
    pub prr: PRR,
    _reserved12: [u8; 0x01],
    #[doc = "0x34 - Oscillator Calibration Value"]
    pub osccal: OSCCAL,
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
#[doc = "PORTCR (rw) register accessor: an alias for `Reg<PORTCR_SPEC>`"]
pub type PORTCR = crate::Reg<portcr::PORTCR_SPEC>;
#[doc = "Port Configuration Register"]
pub mod portcr;
#[doc = "PRR (rw) register accessor: an alias for `Reg<PRR_SPEC>`"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "Sleep Mode Control Register"]
pub mod smcr;
#[doc = "SPH (rw) register accessor: an alias for `Reg<SPH_SPEC>`"]
pub type SPH = crate::Reg<sph::SPH_SPEC>;
#[doc = "Stack Pointer High"]
pub mod sph;
#[doc = "SPL (rw) register accessor: an alias for `Reg<SPL_SPEC>`"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low"]
pub mod spl;
#[doc = "SPMCSR (rw) register accessor: an alias for `Reg<SPMCSR_SPEC>`"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control Register"]
pub mod spmcsr;
