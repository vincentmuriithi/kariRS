#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose I/O Register 0"]
    pub gpior0: GPIOR0,
    #[doc = "0x01 - General Purpose I/O Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x02 - General Purpose I/O Register 2"]
    pub gpior2: GPIOR2,
    _reserved3: [u8; 0x0a],
    #[doc = "0x0d - Pin-Change Mask register"]
    pub pcmsk: PCMSK,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - General Timer Counter Control Register"]
    pub gtccr: GTCCR,
    _reserved5: [u8; 0x02],
    #[doc = "0x13 - Clock Prescale Register"]
    pub clkpr: CLKPR,
    _reserved6: [u8; 0x0a],
    #[doc = "0x1e - Oscillator Calibration Register"]
    pub osccal: OSCCAL,
    _reserved7: [u8; 0x02],
    #[doc = "0x21 - MCU Status register"]
    pub mcusr: MCUSR,
    #[doc = "0x22 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved9: [u8; 0x01],
    #[doc = "0x24 - Store Program Memory Control and Status register"]
    pub spmcsr: SPMCSR,
    _reserved10: [u8; 0x05],
    #[doc = "0x2a - Stack Pointer Low Byte"]
    pub spl: SPL,
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
#[doc = "PCMSK (rw) register accessor: an alias for `Reg<PCMSK_SPEC>`"]
pub type PCMSK = crate::Reg<pcmsk::PCMSK_SPEC>;
#[doc = "Pin-Change Mask register"]
pub mod pcmsk;
#[doc = "SPL (rw) register accessor: an alias for `Reg<SPL_SPEC>`"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low Byte"]
pub mod spl;
#[doc = "SPMCSR (rw) register accessor: an alias for `Reg<SPMCSR_SPEC>`"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status register"]
pub mod spmcsr;
