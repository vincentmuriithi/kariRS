#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description."]
    pub low: LOW,
    #[doc = "0x01 - No Description."]
    pub high: HIGH,
}
#[doc = "HIGH (rw) register accessor: an alias for `Reg<HIGH_SPEC>`"]
pub type HIGH = crate::Reg<high::HIGH_SPEC>;
#[doc = "No Description."]
pub mod high;
#[doc = "LOW (rw) register accessor: an alias for `Reg<LOW_SPEC>`"]
pub type LOW = crate::Reg<low::LOW_SPEC>;
#[doc = "No Description."]
pub mod low;
