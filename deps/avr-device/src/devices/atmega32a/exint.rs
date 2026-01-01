#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Control And Status Register"]
    pub mcucsr: MCUCSR,
    #[doc = "0x01 - General Interrupt Control Register"]
    pub mcucr: MCUCR,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - General Interrupt Flag Register"]
    pub gifr: GIFR,
    #[doc = "0x07 - General Interrupt Control Register"]
    pub gicr: GICR,
}
#[doc = "GICR (rw) register accessor: an alias for `Reg<GICR_SPEC>`"]
pub type GICR = crate::Reg<gicr::GICR_SPEC>;
#[doc = "General Interrupt Control Register"]
pub mod gicr;
#[doc = "GIFR (rw) register accessor: an alias for `Reg<GIFR_SPEC>`"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag Register"]
pub mod gifr;
#[doc = "MCUCR (rw) register accessor: an alias for `Reg<MCUCR_SPEC>`"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "General Interrupt Control Register"]
pub mod mcucr;
#[doc = "MCUCSR (rw) register accessor: an alias for `Reg<MCUCSR_SPEC>`"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
