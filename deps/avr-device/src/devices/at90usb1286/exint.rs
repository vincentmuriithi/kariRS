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
    #[doc = "0x2e - External Interrupt Control Register A"]
    pub eicra: EICRA,
    #[doc = "0x2f - External Interrupt Control Register B"]
    pub eicrb: EICRB,
    #[doc = "0x30 - Pin Change Mask Register 0"]
    pub pcmsk0: PCMSK0,
}
#[doc = "EICRA (rw) register accessor: an alias for `Reg<EICRA_SPEC>`"]
pub type EICRA = crate::Reg<eicra::EICRA_SPEC>;
#[doc = "External Interrupt Control Register A"]
pub mod eicra;
#[doc = "EICRB (rw) register accessor: an alias for `Reg<EICRB_SPEC>`"]
pub type EICRB = crate::Reg<eicrb::EICRB_SPEC>;
#[doc = "External Interrupt Control Register B"]
pub mod eicrb;
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
