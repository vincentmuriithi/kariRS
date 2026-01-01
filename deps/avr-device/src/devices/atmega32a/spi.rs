#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    pub spcr: SPCR,
    #[doc = "0x01 - SPI Status Register"]
    pub spsr: SPSR,
    #[doc = "0x02 - SPI Data Register"]
    pub spdr: SPDR,
}
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
