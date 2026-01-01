#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Multiplexer EVSYS"]
    pub evsysroutea: EVSYSROUTEA,
    #[doc = "0x01 - Port Multiplexer CCL"]
    pub cclroutea: CCLROUTEA,
    #[doc = "0x02 - Port Multiplexer USART register A"]
    pub usartroutea: USARTROUTEA,
    #[doc = "0x03 - Port Multiplexer TWI and SPI"]
    pub twispiroutea: TWISPIROUTEA,
    #[doc = "0x04 - Port Multiplexer TCA"]
    pub tcaroutea: TCAROUTEA,
    #[doc = "0x05 - Port Multiplexer TCB"]
    pub tcbroutea: TCBROUTEA,
}
#[doc = "CCLROUTEA (rw) register accessor: an alias for `Reg<CCLROUTEA_SPEC>`"]
pub type CCLROUTEA = crate::Reg<cclroutea::CCLROUTEA_SPEC>;
#[doc = "Port Multiplexer CCL"]
pub mod cclroutea;
#[doc = "EVSYSROUTEA (rw) register accessor: an alias for `Reg<EVSYSROUTEA_SPEC>`"]
pub type EVSYSROUTEA = crate::Reg<evsysroutea::EVSYSROUTEA_SPEC>;
#[doc = "Port Multiplexer EVSYS"]
pub mod evsysroutea;
#[doc = "TCAROUTEA (rw) register accessor: an alias for `Reg<TCAROUTEA_SPEC>`"]
pub type TCAROUTEA = crate::Reg<tcaroutea::TCAROUTEA_SPEC>;
#[doc = "Port Multiplexer TCA"]
pub mod tcaroutea;
#[doc = "TCBROUTEA (rw) register accessor: an alias for `Reg<TCBROUTEA_SPEC>`"]
pub type TCBROUTEA = crate::Reg<tcbroutea::TCBROUTEA_SPEC>;
#[doc = "Port Multiplexer TCB"]
pub mod tcbroutea;
#[doc = "TWISPIROUTEA (rw) register accessor: an alias for `Reg<TWISPIROUTEA_SPEC>`"]
pub type TWISPIROUTEA = crate::Reg<twispiroutea::TWISPIROUTEA_SPEC>;
#[doc = "Port Multiplexer TWI and SPI"]
pub mod twispiroutea;
#[doc = "USARTROUTEA (rw) register accessor: an alias for `Reg<USARTROUTEA_SPEC>`"]
pub type USARTROUTEA = crate::Reg<usartroutea::USARTROUTEA_SPEC>;
#[doc = "Port Multiplexer USART register A"]
pub mod usartroutea;
