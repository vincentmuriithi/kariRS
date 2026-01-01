#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Remap Port Pins"]
    pub remap: REMAP,
    _reserved1: [u8; 0x4a],
    #[doc = "0x4b - SPI Data Register"]
    pub spdr: SPDR,
    #[doc = "0x4c - SPI Status Register"]
    pub spsr: SPSR,
    #[doc = "0x4d - SPI Control Register"]
    pub spcr: SPCR,
}
#[doc = "REMAP (rw) register accessor: an alias for `Reg<REMAP_SPEC>`"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Remap Port Pins"]
pub mod remap;
#[doc = "SPCR (rw) register accessor: an alias for `Reg<SPCR_SPEC>`"]
pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr;
#[doc = "SPDR (rw) register accessor: an alias for `Reg<SPDR_SPEC>`"]
pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr;
#[doc = "SPSR (rw) register accessor: an alias for `Reg<SPSR_SPEC>`"]
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr;
