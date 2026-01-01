#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port G Input Pins Address"]
    pub ping: PING,
    #[doc = "0x01 - Port G Data Direction Register"]
    pub ddrg: DDRG,
    #[doc = "0x02 - Port G Data Register"]
    pub portg: PORTG,
}
#[doc = "DDRG (rw) register accessor: an alias for `Reg<DDRG_SPEC>`"]
pub type DDRG = crate::Reg<ddrg::DDRG_SPEC>;
#[doc = "Port G Data Direction Register"]
pub mod ddrg;
#[doc = "PING (rw) register accessor: an alias for `Reg<PING_SPEC>`"]
pub type PING = crate::Reg<ping::PING_SPEC>;
#[doc = "Port G Input Pins Address"]
pub mod ping;
#[doc = "PORTG (rw) register accessor: an alias for `Reg<PORTG_SPEC>`"]
pub type PORTG = crate::Reg<portg::PORTG_SPEC>;
#[doc = "Port G Data Register"]
pub mod portg;
