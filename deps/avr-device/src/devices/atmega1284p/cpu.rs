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
    _reserved6: [u8; 0x05],
    #[doc = "0x1d - RAM Page Z Select Register"]
    pub rampz: RAMPZ,
    _reserved7: [u8; 0x05],
    #[doc = "0x23 - No Description."]
    pub clkpr: CLKPR,
    _reserved8: [u8; 0x02],
    #[doc = "0x26 - Power Reduction Register0"]
    pub prr0: PRR0,
    #[doc = "0x27 - Power Reduction Register1"]
    pub prr1: PRR1,
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
#[doc = "PRR0 (rw) register accessor: an alias for `Reg<PRR0_SPEC>`"]
pub type PRR0 = crate::Reg<prr0::PRR0_SPEC>;
#[doc = "Power Reduction Register0"]
pub mod prr0;
#[doc = "PRR1 (rw) register accessor: an alias for `Reg<PRR1_SPEC>`"]
pub type PRR1 = crate::Reg<prr1::PRR1_SPEC>;
#[doc = "Power Reduction Register1"]
pub mod prr1;
#[doc = "RAMPZ (rw) register accessor: an alias for `Reg<RAMPZ_SPEC>`"]
pub type RAMPZ = crate::Reg<rampz::RAMPZ_SPEC>;
#[doc = "RAM Page Z Select Register"]
pub mod rampz;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "Sleep Mode Control Register"]
pub mod smcr;
