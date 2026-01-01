#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port E Input Pins"]
    pub pine: PINE,
    #[doc = "0x01 - Port E Data Direction Register"]
    pub ddre: DDRE,
    #[doc = "0x02 - Port E Data Register"]
    pub porte: PORTE,
}
#[doc = "DDRE (rw) register accessor: an alias for `Reg<DDRE_SPEC>`"]
pub type DDRE = crate::Reg<ddre::DDRE_SPEC>;
#[doc = "Port E Data Direction Register"]
pub mod ddre;
#[doc = "PINE (rw) register accessor: an alias for `Reg<PINE_SPEC>`"]
pub type PINE = crate::Reg<pine::PINE_SPEC>;
#[doc = "Port E Input Pins"]
pub mod pine;
#[doc = "PORTE (rw) register accessor: an alias for `Reg<PORTE_SPEC>`"]
pub type PORTE = crate::Reg<porte::PORTE_SPEC>;
#[doc = "Port E Data Register"]
pub mod porte;
