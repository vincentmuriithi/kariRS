#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose IO Register 0"]
    pub gpior0: GPIOR0,
    #[doc = "0x01 - General Purpose IO Register 1"]
    pub gpior1: GPIOR1,
    #[doc = "0x02 - General Purpose IO Register 2"]
    pub gpior2: GPIOR2,
    #[doc = "0x03 - General Purpose IO Register 3"]
    pub gpior3: GPIOR3,
}
#[doc = "GPIOR0 (rw) register accessor: an alias for `Reg<GPIOR0_SPEC>`"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General Purpose IO Register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: an alias for `Reg<GPIOR1_SPEC>`"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose IO Register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: an alias for `Reg<GPIOR2_SPEC>`"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose IO Register 2"]
pub mod gpior2;
#[doc = "GPIOR3 (rw) register accessor: an alias for `Reg<GPIOR3_SPEC>`"]
pub type GPIOR3 = crate::Reg<gpior3::GPIOR3_SPEC>;
#[doc = "General Purpose IO Register 3"]
pub mod gpior3;
