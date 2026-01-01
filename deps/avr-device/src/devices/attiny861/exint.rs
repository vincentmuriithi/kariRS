#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Change Enable Mask 1"]
    pub pcmsk1: PCMSK1,
    #[doc = "0x01 - Pin Change Enable Mask 0"]
    pub pcmsk0: PCMSK0,
    _reserved2: [u8; 0x11],
    #[doc = "0x13 - MCU Control Register"]
    pub mcucr: MCUCR,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - General Interrupt Flag register"]
    pub gifr: GIFR,
    #[doc = "0x19 - General Interrupt Mask Register"]
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
#[doc = "PCMSK0 (rw) register accessor: an alias for `Reg<PCMSK0_SPEC>`"]
pub type PCMSK0 = crate::Reg<pcmsk0::PCMSK0_SPEC>;
#[doc = "Pin Change Enable Mask 0"]
pub mod pcmsk0;
#[doc = "PCMSK1 (rw) register accessor: an alias for `Reg<PCMSK1_SPEC>`"]
pub type PCMSK1 = crate::Reg<pcmsk1::PCMSK1_SPEC>;
#[doc = "Pin Change Enable Mask 1"]
pub mod pcmsk1;
