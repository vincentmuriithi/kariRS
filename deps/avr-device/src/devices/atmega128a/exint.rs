#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Interrupt Flag Register"]
    pub eifr: EIFR,
    #[doc = "0x01 - External Interrupt Mask Register"]
    pub eimsk: EIMSK,
    #[doc = "0x02 - External Interrupt Control Register B"]
    pub eicrb: EICRB,
    _reserved3: [u8; 0x0f],
    #[doc = "0x12 - External Interrupt Control Register A"]
    pub eicra: EICRA,
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
