#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Sequential Control 0"]
    pub seqctrl0: SEQCTRL0,
    #[doc = "0x02 - Sequential Control 1"]
    pub seqctrl1: SEQCTRL1,
    _reserved3: [u8; 0x02],
    #[doc = "0x05 - Interrupt Control 0"]
    pub intctrl0: INTCTRL0,
    _reserved4: [u8; 0x01],
    #[doc = "0x07 - Interrupt Flags"]
    pub intflags: INTFLAGS,
    #[doc = "0x08 - LUT Control 0 A"]
    pub lut0ctrla: LUT0CTRLA,
    #[doc = "0x09 - LUT Control 0 B"]
    pub lut0ctrlb: LUT0CTRLB,
    #[doc = "0x0a - LUT Control 0 C"]
    pub lut0ctrlc: LUT0CTRLC,
    #[doc = "0x0b - Truth 0"]
    pub truth0: TRUTH0,
    #[doc = "0x0c - LUT Control 1 A"]
    pub lut1ctrla: LUT1CTRLA,
    #[doc = "0x0d - LUT Control 1 B"]
    pub lut1ctrlb: LUT1CTRLB,
    #[doc = "0x0e - LUT Control 1 C"]
    pub lut1ctrlc: LUT1CTRLC,
    #[doc = "0x0f - Truth 1"]
    pub truth1: TRUTH1,
    #[doc = "0x10 - LUT Control 2 A"]
    pub lut2ctrla: LUT2CTRLA,
    #[doc = "0x11 - LUT Control 2 B"]
    pub lut2ctrlb: LUT2CTRLB,
    #[doc = "0x12 - LUT Control 2 C"]
    pub lut2ctrlc: LUT2CTRLC,
    #[doc = "0x13 - Truth 2"]
    pub truth2: TRUTH2,
    #[doc = "0x14 - LUT Control 3 A"]
    pub lut3ctrla: LUT3CTRLA,
    #[doc = "0x15 - LUT Control 3 B"]
    pub lut3ctrlb: LUT3CTRLB,
    #[doc = "0x16 - LUT Control 3 C"]
    pub lut3ctrlc: LUT3CTRLC,
    #[doc = "0x17 - Truth 3"]
    pub truth3: TRUTH3,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "INTCTRL0 (rw) register accessor: an alias for `Reg<INTCTRL0_SPEC>`"]
pub type INTCTRL0 = crate::Reg<intctrl0::INTCTRL0_SPEC>;
#[doc = "Interrupt Control 0"]
pub mod intctrl0;
#[doc = "INTFLAGS (rw) register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "LUT0CTRLA (rw) register accessor: an alias for `Reg<LUT0CTRLA_SPEC>`"]
pub type LUT0CTRLA = crate::Reg<lut0ctrla::LUT0CTRLA_SPEC>;
#[doc = "LUT Control 0 A"]
pub mod lut0ctrla;
#[doc = "LUT0CTRLB (rw) register accessor: an alias for `Reg<LUT0CTRLB_SPEC>`"]
pub type LUT0CTRLB = crate::Reg<lut0ctrlb::LUT0CTRLB_SPEC>;
#[doc = "LUT Control 0 B"]
pub mod lut0ctrlb;
#[doc = "LUT0CTRLC (rw) register accessor: an alias for `Reg<LUT0CTRLC_SPEC>`"]
pub type LUT0CTRLC = crate::Reg<lut0ctrlc::LUT0CTRLC_SPEC>;
#[doc = "LUT Control 0 C"]
pub mod lut0ctrlc;
#[doc = "LUT1CTRLA (rw) register accessor: an alias for `Reg<LUT1CTRLA_SPEC>`"]
pub type LUT1CTRLA = crate::Reg<lut1ctrla::LUT1CTRLA_SPEC>;
#[doc = "LUT Control 1 A"]
pub mod lut1ctrla;
#[doc = "LUT1CTRLB (rw) register accessor: an alias for `Reg<LUT1CTRLB_SPEC>`"]
pub type LUT1CTRLB = crate::Reg<lut1ctrlb::LUT1CTRLB_SPEC>;
#[doc = "LUT Control 1 B"]
pub mod lut1ctrlb;
#[doc = "LUT1CTRLC (rw) register accessor: an alias for `Reg<LUT1CTRLC_SPEC>`"]
pub type LUT1CTRLC = crate::Reg<lut1ctrlc::LUT1CTRLC_SPEC>;
#[doc = "LUT Control 1 C"]
pub mod lut1ctrlc;
#[doc = "LUT2CTRLA (rw) register accessor: an alias for `Reg<LUT2CTRLA_SPEC>`"]
pub type LUT2CTRLA = crate::Reg<lut2ctrla::LUT2CTRLA_SPEC>;
#[doc = "LUT Control 2 A"]
pub mod lut2ctrla;
#[doc = "LUT2CTRLB (rw) register accessor: an alias for `Reg<LUT2CTRLB_SPEC>`"]
pub type LUT2CTRLB = crate::Reg<lut2ctrlb::LUT2CTRLB_SPEC>;
#[doc = "LUT Control 2 B"]
pub mod lut2ctrlb;
#[doc = "LUT2CTRLC (rw) register accessor: an alias for `Reg<LUT2CTRLC_SPEC>`"]
pub type LUT2CTRLC = crate::Reg<lut2ctrlc::LUT2CTRLC_SPEC>;
#[doc = "LUT Control 2 C"]
pub mod lut2ctrlc;
#[doc = "LUT3CTRLA (rw) register accessor: an alias for `Reg<LUT3CTRLA_SPEC>`"]
pub type LUT3CTRLA = crate::Reg<lut3ctrla::LUT3CTRLA_SPEC>;
#[doc = "LUT Control 3 A"]
pub mod lut3ctrla;
#[doc = "LUT3CTRLB (rw) register accessor: an alias for `Reg<LUT3CTRLB_SPEC>`"]
pub type LUT3CTRLB = crate::Reg<lut3ctrlb::LUT3CTRLB_SPEC>;
#[doc = "LUT Control 3 B"]
pub mod lut3ctrlb;
#[doc = "LUT3CTRLC (rw) register accessor: an alias for `Reg<LUT3CTRLC_SPEC>`"]
pub type LUT3CTRLC = crate::Reg<lut3ctrlc::LUT3CTRLC_SPEC>;
#[doc = "LUT Control 3 C"]
pub mod lut3ctrlc;
#[doc = "SEQCTRL0 (rw) register accessor: an alias for `Reg<SEQCTRL0_SPEC>`"]
pub type SEQCTRL0 = crate::Reg<seqctrl0::SEQCTRL0_SPEC>;
#[doc = "Sequential Control 0"]
pub mod seqctrl0;
#[doc = "SEQCTRL1 (rw) register accessor: an alias for `Reg<SEQCTRL1_SPEC>`"]
pub type SEQCTRL1 = crate::Reg<seqctrl1::SEQCTRL1_SPEC>;
#[doc = "Sequential Control 1"]
pub mod seqctrl1;
#[doc = "TRUTH0 (rw) register accessor: an alias for `Reg<TRUTH0_SPEC>`"]
pub type TRUTH0 = crate::Reg<truth0::TRUTH0_SPEC>;
#[doc = "Truth 0"]
pub mod truth0;
#[doc = "TRUTH1 (rw) register accessor: an alias for `Reg<TRUTH1_SPEC>`"]
pub type TRUTH1 = crate::Reg<truth1::TRUTH1_SPEC>;
#[doc = "Truth 1"]
pub mod truth1;
#[doc = "TRUTH2 (rw) register accessor: an alias for `Reg<TRUTH2_SPEC>`"]
pub type TRUTH2 = crate::Reg<truth2::TRUTH2_SPEC>;
#[doc = "Truth 2"]
pub mod truth2;
#[doc = "TRUTH3 (rw) register accessor: an alias for `Reg<TRUTH3_SPEC>`"]
pub type TRUTH3 = crate::Reg<truth3::TRUTH3_SPEC>;
#[doc = "Truth 3"]
pub mod truth3;
