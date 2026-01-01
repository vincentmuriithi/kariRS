#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port A Input Pins"]
    pub pina: PINA,
    #[doc = "0x01 - Data Direction Register, Port A"]
    pub ddra: DDRA,
    #[doc = "0x02 - Port A Data Register"]
    pub porta: PORTA,
    _reserved3: [u8; 0x27],
    #[doc = "0x2a - Pull-up Enable Control Register"]
    pub puea: PUEA,
    #[doc = "0x2b - Port Control Register"]
    pub portcr: PORTCR,
    _reserved5: [u8; 0x05],
    #[doc = "0x31 - Port High Drive Enable Register"]
    pub phde: PHDE,
}
#[doc = "DDRA (rw) register accessor: an alias for `Reg<DDRA_SPEC>`"]
pub type DDRA = crate::Reg<ddra::DDRA_SPEC>;
#[doc = "Data Direction Register, Port A"]
pub mod ddra;
#[doc = "PHDE (rw) register accessor: an alias for `Reg<PHDE_SPEC>`"]
pub type PHDE = crate::Reg<phde::PHDE_SPEC>;
#[doc = "Port High Drive Enable Register"]
pub mod phde;
#[doc = "PINA (rw) register accessor: an alias for `Reg<PINA_SPEC>`"]
pub type PINA = crate::Reg<pina::PINA_SPEC>;
#[doc = "Port A Input Pins"]
pub mod pina;
#[doc = "PORTA (rw) register accessor: an alias for `Reg<PORTA_SPEC>`"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "Port A Data Register"]
pub mod porta;
#[doc = "PORTCR (rw) register accessor: an alias for `Reg<PORTCR_SPEC>`"]
pub type PORTCR = crate::Reg<portcr::PORTCR_SPEC>;
#[doc = "Port Control Register"]
pub mod portcr;
#[doc = "PUEA (rw) register accessor: an alias for `Reg<PUEA_SPEC>`"]
pub type PUEA = crate::Reg<puea::PUEA_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod puea;
