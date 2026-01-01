#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status Register"]
    pub osccal: OSCCAL,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - MCU Status register"]
    pub mcusr: MCUSR,
    #[doc = "0x04 - MCU Control Register"]
    pub mcucr: MCUCR,
}
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
#[doc = "Status Register"]
pub mod osccal;
