#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - On-Chip Debug Related Register in I/O Memory"]
    pub ocdr: OCDR,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - MCU Control And Status Register"]
    pub mcucsr: MCUCSR,
}
#[doc = "MCUCSR (rw) register accessor: an alias for `Reg<MCUCSR_SPEC>`"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
#[doc = "OCDR (rw) register accessor: an alias for `Reg<OCDR_SPEC>`"]
pub type OCDR = crate::Reg<ocdr::OCDR_SPEC>;
#[doc = "On-Chip Debug Related Register in I/O Memory"]
pub mod ocdr;
