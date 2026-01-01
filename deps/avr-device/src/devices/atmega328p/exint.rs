#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Change Interrupt Flag Register"]
    pub pcifr: PCIFR,
    #[doc = "0x01 - External Interrupt Flag Register"]
    pub eifr: EIFR,
    #[doc = "0x02 - External Interrupt Mask Register"]
    pub eimsk: EIMSK,
    _reserved3: [u8; 0x2a],
    #[doc = "0x2d - Pin Change Interrupt Control Register"]
    pub pcicr: PCICR,
    #[doc = "0x2e - External Interrupt Control Register"]
    pub eicra: EICRA,
    _reserved5: [u8; 0x01],
    #[doc = "0x30 - Pin Change Mask Register 0"]
    pub pcmsk0: PCMSK0,
    #[doc = "0x31 - Pin Change Mask Register 1"]
    pub pcmsk1: PCMSK1,
    #[doc = "0x32 - Pin Change Mask Register 2"]
    pub pcmsk2: PCMSK2,
}
#[doc = "EICRA (rw) register accessor: an alias for `Reg<EICRA_SPEC>`"]
pub type EICRA = crate::Reg<eicra::EICRA_SPEC>;
#[doc = "External Interrupt Control Register"]
pub mod eicra;
#[doc = "EIFR (rw) register accessor: an alias for `Reg<EIFR_SPEC>`"]
pub type EIFR = crate::Reg<eifr::EIFR_SPEC>;
#[doc = "External Interrupt Flag Register"]
pub mod eifr;
#[doc = "EIMSK (rw) register accessor: an alias for `Reg<EIMSK_SPEC>`"]
pub type EIMSK = crate::Reg<eimsk::EIMSK_SPEC>;
#[doc = "External Interrupt Mask Register"]
pub mod eimsk;
#[doc = "PCICR (rw) register accessor: an alias for `Reg<PCICR_SPEC>`"]
pub type PCICR = crate::Reg<pcicr::PCICR_SPEC>;
#[doc = "Pin Change Interrupt Control Register"]
pub mod pcicr;
#[doc = "PCIFR (rw) register accessor: an alias for `Reg<PCIFR_SPEC>`"]
pub type PCIFR = crate::Reg<pcifr::PCIFR_SPEC>;
#[doc = "Pin Change Interrupt Flag Register"]
pub mod pcifr;
#[doc = "PCMSK0 (rw) register accessor: an alias for `Reg<PCMSK0_SPEC>`"]
pub type PCMSK0 = crate::Reg<pcmsk0::PCMSK0_SPEC>;
#[doc = "Pin Change Mask Register 0"]
pub mod pcmsk0;
#[doc = "PCMSK1 (rw) register accessor: an alias for `Reg<PCMSK1_SPEC>`"]
pub type PCMSK1 = crate::Reg<pcmsk1::PCMSK1_SPEC>;
#[doc = "Pin Change Mask Register 1"]
pub mod pcmsk1;
#[doc = "PCMSK2 (rw) register accessor: an alias for `Reg<PCMSK2_SPEC>`"]
pub type PCMSK2 = crate::Reg<pcmsk2::PCMSK2_SPEC>;
#[doc = "Pin Change Mask Register 2"]
pub mod pcmsk2;
