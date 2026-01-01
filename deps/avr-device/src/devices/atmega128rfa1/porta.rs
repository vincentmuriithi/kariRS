#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port A Input Pins Address"]
    pub pina: PINA,
    #[doc = "0x01 - Port A Data Direction Register"]
    pub ddra: DDRA,
    #[doc = "0x02 - Port A Data Register"]
    pub porta: PORTA,
}
#[doc = "DDRA (rw) register accessor: an alias for `Reg<DDRA_SPEC>`"]
pub type DDRA = crate::Reg<ddra::DDRA_SPEC>;
#[doc = "Port A Data Direction Register"]
pub mod ddra;
#[doc = "PINA (rw) register accessor: an alias for `Reg<PINA_SPEC>`"]
pub type PINA = crate::Reg<pina::PINA_SPEC>;
#[doc = "Port A Input Pins Address"]
pub mod pina;
#[doc = "PORTA (rw) register accessor: an alias for `Reg<PORTA_SPEC>`"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "Port A Data Register"]
pub mod porta;
