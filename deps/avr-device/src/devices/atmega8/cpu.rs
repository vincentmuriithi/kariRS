#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Special Function IO Register"]
    pub sfior: SFIOR,
    #[doc = "0x01 - Oscillator Calibration Value"]
    pub osccal: OSCCAL,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - MCU Control And Status Register"]
    pub mcucsr: MCUCSR,
    #[doc = "0x05 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved4: [u8; 0x01],
    #[doc = "0x07 - Store Program Memory Control Register"]
    pub spmcr: SPMCR,
}
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUCSR (rw) register accessor: an alias for `Reg<MCUCSR_SPEC>`"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
#[doc = "OSCCAL (rw) register accessor: an alias for `Reg<OSCCAL_SPEC>`"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Value"]
pub mod osccal;
#[doc = "SFIOR (rw) register accessor: an alias for `Reg<SFIOR_SPEC>`"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
#[doc = "SPMCR (rw) register accessor: an alias for `Reg<SPMCR_SPEC>`"]
pub type SPMCR = crate::Reg<spmcr::SPMCR_SPEC>;
#[doc = "Store Program Memory Control Register"]
pub mod spmcr;
