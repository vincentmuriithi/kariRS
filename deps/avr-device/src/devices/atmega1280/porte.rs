#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Pins, Port E"]
    pub pine: PINE,
    #[doc = "0x01 - Data Direction Register, Port E"]
    pub ddre: DDRE,
    #[doc = "0x02 - Data Register, Port E"]
    pub porte: PORTE,
}
#[doc = "DDRE (rw) register accessor: an alias for `Reg<DDRE_SPEC>`"]
pub type DDRE = crate::Reg<ddre::DDRE_SPEC>;
#[doc = "Data Direction Register, Port E"]
pub mod ddre;
#[doc = "PINE (rw) register accessor: an alias for `Reg<PINE_SPEC>`"]
pub type PINE = crate::Reg<pine::PINE_SPEC>;
#[doc = "Input Pins, Port E"]
pub mod pine;
#[doc = "PORTE (rw) register accessor: an alias for `Reg<PORTE_SPEC>`"]
pub type PORTE = crate::Reg<porte::PORTE_SPEC>;
#[doc = "Data Register, Port E"]
pub mod porte;
