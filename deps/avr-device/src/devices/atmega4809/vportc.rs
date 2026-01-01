#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir: DIR,
    #[doc = "0x01 - Output Value"]
    pub out: OUT,
    #[doc = "0x02 - Input Value"]
    pub in_: IN,
    #[doc = "0x03 - Interrupt Flags"]
    pub intflags: INTFLAGS,
}
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "IN (rw) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Input Value"]
pub mod in_;
#[doc = "INTFLAGS (rw) register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Output Value"]
pub mod out;
