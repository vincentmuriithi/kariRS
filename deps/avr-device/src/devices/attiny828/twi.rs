#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TWI Slave Control Register A"]
    pub twscra: TWSCRA,
    #[doc = "0x01 - TWI Slave Control Register B"]
    pub twscrb: TWSCRB,
    #[doc = "0x02 - TWI Slave Status Register A"]
    pub twssra: TWSSRA,
    #[doc = "0x03 - TWI Slave Address Mask Register"]
    pub twsam: TWSAM,
    #[doc = "0x04 - TWI Slave Address Register"]
    pub twsa: TWSA,
    #[doc = "0x05 - TWI Slave Data Register"]
    pub twsd: TWSD,
}
#[doc = "TWSA (rw) register accessor: an alias for `Reg<TWSA_SPEC>`"]
pub type TWSA = crate::Reg<twsa::TWSA_SPEC>;
#[doc = "TWI Slave Address Register"]
pub mod twsa;
#[doc = "TWSAM (rw) register accessor: an alias for `Reg<TWSAM_SPEC>`"]
pub type TWSAM = crate::Reg<twsam::TWSAM_SPEC>;
#[doc = "TWI Slave Address Mask Register"]
pub mod twsam;
#[doc = "TWSCRA (rw) register accessor: an alias for `Reg<TWSCRA_SPEC>`"]
pub type TWSCRA = crate::Reg<twscra::TWSCRA_SPEC>;
#[doc = "TWI Slave Control Register A"]
pub mod twscra;
#[doc = "TWSCRB (rw) register accessor: an alias for `Reg<TWSCRB_SPEC>`"]
pub type TWSCRB = crate::Reg<twscrb::TWSCRB_SPEC>;
#[doc = "TWI Slave Control Register B"]
pub mod twscrb;
#[doc = "TWSD (rw) register accessor: an alias for `Reg<TWSD_SPEC>`"]
pub type TWSD = crate::Reg<twsd::TWSD_SPEC>;
#[doc = "TWI Slave Data Register"]
pub mod twsd;
#[doc = "TWSSRA (rw) register accessor: an alias for `Reg<TWSSRA_SPEC>`"]
pub type TWSSRA = crate::Reg<twssra::TWSSRA_SPEC>;
#[doc = "TWI Slave Status Register A"]
pub mod twssra;
