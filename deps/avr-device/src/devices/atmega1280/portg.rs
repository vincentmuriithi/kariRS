#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Pins, Port G"]
    pub ping: PING,
    #[doc = "0x01 - Data Direction Register, Port G"]
    pub ddrg: DDRG,
    #[doc = "0x02 - Data Register, Port G"]
    pub portg: PORTG,
}
#[doc = "DDRG (rw) register accessor: an alias for `Reg<DDRG_SPEC>`"]
pub type DDRG = crate::Reg<ddrg::DDRG_SPEC>;
#[doc = "Data Direction Register, Port G"]
pub mod ddrg;
#[doc = "PING (rw) register accessor: an alias for `Reg<PING_SPEC>`"]
pub type PING = crate::Reg<ping::PING_SPEC>;
#[doc = "Input Pins, Port G"]
pub mod ping;
#[doc = "PORTG (rw) register accessor: an alias for `Reg<PORTG_SPEC>`"]
pub type PORTG = crate::Reg<portg::PORTG_SPEC>;
#[doc = "Data Register, Port G"]
pub mod portg;
