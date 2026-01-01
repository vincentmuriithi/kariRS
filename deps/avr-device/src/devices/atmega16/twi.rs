#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TWI Bit Rate register"]
    pub twbr: TWBR,
    #[doc = "0x01 - TWI Status Register"]
    pub twsr: TWSR,
    #[doc = "0x02 - TWI (Slave) Address register"]
    pub twar: TWAR,
    #[doc = "0x03 - TWI Data register"]
    pub twdr: TWDR,
    _reserved4: [u8; 0x32],
    #[doc = "0x36 - TWI Control Register"]
    pub twcr: TWCR,
}
#[doc = "TWAR (rw) register accessor: an alias for `Reg<TWAR_SPEC>`"]
pub type TWAR = crate::Reg<twar::TWAR_SPEC>;
#[doc = "TWI (Slave) Address register"]
pub mod twar;
#[doc = "TWBR (rw) register accessor: an alias for `Reg<TWBR_SPEC>`"]
pub type TWBR = crate::Reg<twbr::TWBR_SPEC>;
#[doc = "TWI Bit Rate register"]
pub mod twbr;
#[doc = "TWCR (rw) register accessor: an alias for `Reg<TWCR_SPEC>`"]
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
#[doc = "TWI Control Register"]
pub mod twcr;
#[doc = "TWDR (rw) register accessor: an alias for `Reg<TWDR_SPEC>`"]
pub type TWDR = crate::Reg<twdr::TWDR_SPEC>;
#[doc = "TWI Data register"]
pub mod twdr;
#[doc = "TWSR (rw) register accessor: an alias for `Reg<TWSR_SPEC>`"]
pub type TWSR = crate::Reg<twsr::TWSR_SPEC>;
#[doc = "TWI Status Register"]
pub mod twsr;
