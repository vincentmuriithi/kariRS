#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port A Input Pins"]
    pub pina: PINA,
    #[doc = "0x01 - Data Direction Register, Port A"]
    pub ddra: DDRA,
    #[doc = "0x02 - Port A Data Register"]
    pub porta: PORTA,
    #[doc = "0x03 - Pull-up Enable Control Register"]
    pub puea: PUEA,
}
#[doc = "DDRA (rw) register accessor: an alias for `Reg<DDRA_SPEC>`"]
pub type DDRA = crate::Reg<ddra::DDRA_SPEC>;
#[doc = "Data Direction Register, Port A"]
pub mod ddra;
#[doc = "PINA (rw) register accessor: an alias for `Reg<PINA_SPEC>`"]
pub type PINA = crate::Reg<pina::PINA_SPEC>;
#[doc = "Port A Input Pins"]
pub mod pina;
#[doc = "PORTA (rw) register accessor: an alias for `Reg<PORTA_SPEC>`"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "Port A Data Register"]
pub mod porta;
#[doc = "PUEA (rw) register accessor: an alias for `Reg<PUEA_SPEC>`"]
pub type PUEA = crate::Reg<puea::PUEA_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod puea;
