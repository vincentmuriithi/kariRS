#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Control Register"]
    pub eecr: EECR,
    #[doc = "0x01 - EEPROM Data Register"]
    pub eedr: EEDR,
    #[doc = "0x02 - EEPROM Address Register Low Bytes"]
    pub eear: EEAR,
}
#[doc = "EEAR (rw) register accessor: an alias for `Reg<EEAR_SPEC>`"]
pub type EEAR = crate::Reg<eear::EEAR_SPEC>;
#[doc = "EEPROM Address Register Low Bytes"]
pub mod eear;
#[doc = "EECR (rw) register accessor: an alias for `Reg<EECR_SPEC>`"]
pub type EECR = crate::Reg<eecr::EECR_SPEC>;
#[doc = "EEPROM Control Register"]
pub mod eecr;
#[doc = "EEDR (rw) register accessor: an alias for `Reg<EEDR_SPEC>`"]
pub type EEDR = crate::Reg<eedr::EEDR_SPEC>;
#[doc = "EEPROM Data Register"]
pub mod eedr;
