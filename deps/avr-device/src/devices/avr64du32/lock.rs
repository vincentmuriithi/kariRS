#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Key Bits"]
    pub key: KEY,
}
#[doc = "KEY (rw) register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Lock Key Bits"]
pub mod key;
