#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EVSYS route A"]
    pub evsysroutea: EVSYSROUTEA,
    #[doc = "0x01 - CCL route A"]
    pub cclroutea: CCLROUTEA,
    #[doc = "0x02 - USART route A"]
    pub usartroutea: USARTROUTEA,
    _reserved3: [u8; 0x02],
    #[doc = "0x05 - SPI route A"]
    pub spiroutea: SPIROUTEA,
    #[doc = "0x06 - TWI route A"]
    pub twiroutea: TWIROUTEA,
    #[doc = "0x07 - TCA route A"]
    pub tcaroutea: TCAROUTEA,
    #[doc = "0x08 - TCB route A"]
    pub tcbroutea: TCBROUTEA,
}
#[doc = "CCLROUTEA (rw) register accessor: an alias for `Reg<CCLROUTEA_SPEC>`"]
pub type CCLROUTEA = crate::Reg<cclroutea::CCLROUTEA_SPEC>;
#[doc = "CCL route A"]
pub mod cclroutea;
#[doc = "EVSYSROUTEA (rw) register accessor: an alias for `Reg<EVSYSROUTEA_SPEC>`"]
pub type EVSYSROUTEA = crate::Reg<evsysroutea::EVSYSROUTEA_SPEC>;
#[doc = "EVSYS route A"]
pub mod evsysroutea;
#[doc = "SPIROUTEA (rw) register accessor: an alias for `Reg<SPIROUTEA_SPEC>`"]
pub type SPIROUTEA = crate::Reg<spiroutea::SPIROUTEA_SPEC>;
#[doc = "SPI route A"]
pub mod spiroutea;
#[doc = "TCAROUTEA (rw) register accessor: an alias for `Reg<TCAROUTEA_SPEC>`"]
pub type TCAROUTEA = crate::Reg<tcaroutea::TCAROUTEA_SPEC>;
#[doc = "TCA route A"]
pub mod tcaroutea;
#[doc = "TCBROUTEA (rw) register accessor: an alias for `Reg<TCBROUTEA_SPEC>`"]
pub type TCBROUTEA = crate::Reg<tcbroutea::TCBROUTEA_SPEC>;
#[doc = "TCB route A"]
pub mod tcbroutea;
#[doc = "TWIROUTEA (rw) register accessor: an alias for `Reg<TWIROUTEA_SPEC>`"]
pub type TWIROUTEA = crate::Reg<twiroutea::TWIROUTEA_SPEC>;
#[doc = "TWI route A"]
pub mod twiroutea;
#[doc = "USARTROUTEA (rw) register accessor: an alias for `Reg<USARTROUTEA_SPEC>`"]
pub type USARTROUTEA = crate::Reg<usartroutea::USARTROUTEA_SPEC>;
#[doc = "USART route A"]
pub mod usartroutea;
