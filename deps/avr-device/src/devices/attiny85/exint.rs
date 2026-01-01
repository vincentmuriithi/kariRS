#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Change Enable Mask"]
    pub pcmsk: PCMSK,
    _reserved1: [u8; 0x1f],
    #[doc = "0x20 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved2: [u8; 0x04],
    #[doc = "0x25 - General Interrupt Flag register"]
    pub gifr: GIFR,
    #[doc = "0x26 - General Interrupt Mask Register"]
    pub gimsk: GIMSK,
}
#[doc = "GIFR (rw) register accessor: an alias for `Reg<GIFR_SPEC>`"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag register"]
pub mod gifr;
#[doc = "GIMSK (rw) register accessor: an alias for `Reg<GIMSK_SPEC>`"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "PCMSK (rw) register accessor: an alias for `Reg<PCMSK_SPEC>`"]
pub type PCMSK = crate::Reg<pcmsk::PCMSK_SPEC>;
#[doc = "Pin Change Enable Mask"]
pub mod pcmsk;
