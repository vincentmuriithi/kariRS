#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port C Input Pins"]
    pub pinc: PINC,
    #[doc = "0x01 - Data Direction Register, Port C"]
    pub ddrc: DDRC,
    #[doc = "0x02 - Port C Data Register"]
    pub portc: PORTC,
    #[doc = "0x03 - Pull-up Enable Control Register"]
    pub puec: PUEC,
    _reserved4: [u8; 0x08],
    #[doc = "0x0c - Port High Drive Enable Register"]
    pub phde: PHDE,
}
#[doc = "DDRC (rw) register accessor: an alias for `Reg<DDRC_SPEC>`"]
pub type DDRC = crate::Reg<ddrc::DDRC_SPEC>;
#[doc = "Data Direction Register, Port C"]
pub mod ddrc;
#[doc = "PHDE (rw) register accessor: an alias for `Reg<PHDE_SPEC>`"]
pub type PHDE = crate::Reg<phde::PHDE_SPEC>;
#[doc = "Port High Drive Enable Register"]
pub mod phde;
#[doc = "PINC (rw) register accessor: an alias for `Reg<PINC_SPEC>`"]
pub type PINC = crate::Reg<pinc::PINC_SPEC>;
#[doc = "Port C Input Pins"]
pub mod pinc;
#[doc = "PORTC (rw) register accessor: an alias for `Reg<PORTC_SPEC>`"]
pub type PORTC = crate::Reg<portc::PORTC_SPEC>;
#[doc = "Port C Data Register"]
pub mod portc;
#[doc = "PUEC (rw) register accessor: an alias for `Reg<PUEC_SPEC>`"]
pub type PUEC = crate::Reg<puec::PUEC_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod puec;
