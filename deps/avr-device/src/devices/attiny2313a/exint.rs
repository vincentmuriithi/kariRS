#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Change Interrupt Mask Register 1"]
    pub pcmsk1: PCMSK1,
    #[doc = "0x01 - Pin Change Interrupt Mask Register 2"]
    pub pcmsk2: PCMSK2,
    _reserved2: [u8; 0x1a],
    #[doc = "0x1c - Pin Change Interrupt Mask Register 0"]
    pub pcmsk0: PCMSK0,
    _reserved3: [u8; 0x19],
    #[doc = "0x36 - General Interrupt Flag Register"]
    pub gifr: GIFR,
    #[doc = "0x37 - General Interrupt Mask Register"]
    pub gimsk: GIMSK,
}
#[doc = "GIFR (rw) register accessor: an alias for `Reg<GIFR_SPEC>`"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag Register"]
pub mod gifr;
#[doc = "GIMSK (rw) register accessor: an alias for `Reg<GIMSK_SPEC>`"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
#[doc = "PCMSK0 (rw) register accessor: an alias for `Reg<PCMSK0_SPEC>`"]
pub type PCMSK0 = crate::Reg<pcmsk0::PCMSK0_SPEC>;
#[doc = "Pin Change Interrupt Mask Register 0"]
pub mod pcmsk0;
#[doc = "PCMSK1 (rw) register accessor: an alias for `Reg<PCMSK1_SPEC>`"]
pub type PCMSK1 = crate::Reg<pcmsk1::PCMSK1_SPEC>;
#[doc = "Pin Change Interrupt Mask Register 1"]
pub mod pcmsk1;
#[doc = "PCMSK2 (rw) register accessor: an alias for `Reg<PCMSK2_SPEC>`"]
pub type PCMSK2 = crate::Reg<pcmsk2::PCMSK2_SPEC>;
#[doc = "Pin Change Interrupt Mask Register 2"]
pub mod pcmsk2;
