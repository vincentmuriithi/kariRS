#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Control Register"]
    pub eecr: EECR,
    #[doc = "0x01 - EEPROM Data Register"]
    pub eedr: EEDR,
    #[doc = "0x02 - EEPROM Address Register Low Byte"]
    pub eearl: EEARL,
}
#[doc = "EEARL (rw) register accessor: an alias for `Reg<EEARL_SPEC>`"]
pub type EEARL = crate::Reg<eearl::EEARL_SPEC>;
#[doc = "EEPROM Address Register Low Byte"]
pub mod eearl;
#[doc = "EECR (rw) register accessor: an alias for `Reg<EECR_SPEC>`"]
pub type EECR = crate::Reg<eecr::EECR_SPEC>;
#[doc = "EEPROM Control Register"]
pub mod eecr;
#[doc = "EEDR (rw) register accessor: an alias for `Reg<EEDR_SPEC>`"]
pub type EEDR = crate::Reg<eedr::EEDR_SPEC>;
#[doc = "EEPROM Data Register"]
pub mod eedr;
