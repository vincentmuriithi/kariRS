#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PORT L Input Pins"]
    pub pinl: PINL,
    #[doc = "0x01 - PORT L Data Direction Register"]
    pub ddrl: DDRL,
    #[doc = "0x02 - PORT L Data Register"]
    pub portl: PORTL,
}
#[doc = "DDRL (rw) register accessor: an alias for `Reg<DDRL_SPEC>`"]
pub type DDRL = crate::Reg<ddrl::DDRL_SPEC>;
#[doc = "PORT L Data Direction Register"]
pub mod ddrl;
#[doc = "PINL (rw) register accessor: an alias for `Reg<PINL_SPEC>`"]
pub type PINL = crate::Reg<pinl::PINL_SPEC>;
#[doc = "PORT L Input Pins"]
pub mod pinl;
#[doc = "PORTL (rw) register accessor: an alias for `Reg<PORTL_SPEC>`"]
pub type PORTL = crate::Reg<portl::PORTL_SPEC>;
#[doc = "PORT L Data Register"]
pub mod portl;
