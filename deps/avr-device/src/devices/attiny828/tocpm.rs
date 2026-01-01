#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Output Compare Pin Mux Channel Output Enable"]
    pub tocpmcoe: TOCPMCOE,
    _reserved1: [u8; 0x05],
    #[doc = "0x06 - Timer Output Compare Pin Mux Selection 0"]
    pub tocpmsa0: TOCPMSA0,
    #[doc = "0x07 - Timer Output Compare Pin Mux Selection 1"]
    pub tocpmsa1: TOCPMSA1,
}
#[doc = "TOCPMCOE (rw) register accessor: an alias for `Reg<TOCPMCOE_SPEC>`"]
pub type TOCPMCOE = crate::Reg<tocpmcoe::TOCPMCOE_SPEC>;
#[doc = "Timer Output Compare Pin Mux Channel Output Enable"]
pub mod tocpmcoe;
#[doc = "TOCPMSA0 (rw) register accessor: an alias for `Reg<TOCPMSA0_SPEC>`"]
pub type TOCPMSA0 = crate::Reg<tocpmsa0::TOCPMSA0_SPEC>;
#[doc = "Timer Output Compare Pin Mux Selection 0"]
pub mod tocpmsa0;
#[doc = "TOCPMSA1 (rw) register accessor: an alias for `Reg<TOCPMSA1_SPEC>`"]
pub type TOCPMSA1 = crate::Reg<tocpmsa1::TOCPMSA1_SPEC>;
#[doc = "Timer Output Compare Pin Mux Selection 1"]
pub mod tocpmsa1;
