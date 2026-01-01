#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    pub spcr0: SPCR0,
    #[doc = "0x01 - SPI Status Register"]
    pub spsr0: SPSR0,
    #[doc = "0x02 - SPI Data Register"]
    pub spdr0: SPDR0,
}
#[doc = "SPCR0 (rw) register accessor: an alias for `Reg<SPCR0_SPEC>`"]
pub type SPCR0 = crate::Reg<spcr0::SPCR0_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr0;
#[doc = "SPDR0 (rw) register accessor: an alias for `Reg<SPDR0_SPEC>`"]
pub type SPDR0 = crate::Reg<spdr0::SPDR0_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr0;
#[doc = "SPSR0 (rw) register accessor: an alias for `Reg<SPSR0_SPEC>`"]
pub type SPSR0 = crate::Reg<spsr0::SPSR0_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr0;
