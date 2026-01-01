#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter Interrupt Flag register"]
    pub tifr2: TIFR2,
    #[doc = "0x01 - Timer/Counter2 Interrupt Mask Register"]
    pub timsk2: TIMSK2,
    _reserved2: [u8; 0x11],
    #[doc = "0x13 - General Timer/Counter Control Register"]
    pub gtccr: GTCCR,
    _reserved3: [u8; 0x7c],
    #[doc = "0x90 - Timer/Counter2 Input Capture Register Bytes"]
    pub icr2: ICR2,
    #[doc = "0x92 - Timer/Counter2 Output Compare Register B Bytes"]
    pub ocr2b: OCR2B,
    #[doc = "0x94 - Timer/Counter2 Output Compare Register A Bytes"]
    pub ocr2a: OCR2A,
    #[doc = "0x96 - Timer/Counter2 Bytes"]
    pub tcnt2: TCNT2,
    #[doc = "0x98 - Timer/Counter2 Control Register C"]
    pub tccr2c: TCCR2C,
    #[doc = "0x99 - Timer/Counter2 Control Register B"]
    pub tccr2b: TCCR2B,
    #[doc = "0x9a - Timer/Counter2 Control Register A"]
    pub tccr2a: TCCR2A,
}
#[doc = "GTCCR (rw) register accessor: an alias for `Reg<GTCCR_SPEC>`"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer/Counter Control Register"]
pub mod gtccr;
#[doc = "ICR2 (rw) register accessor: an alias for `Reg<ICR2_SPEC>`"]
pub type ICR2 = crate::Reg<icr2::ICR2_SPEC>;
#[doc = "Timer/Counter2 Input Capture Register Bytes"]
pub mod icr2;
#[doc = "OCR2A (rw) register accessor: an alias for `Reg<OCR2A_SPEC>`"]
pub type OCR2A = crate::Reg<ocr2a::OCR2A_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register A Bytes"]
pub mod ocr2a;
#[doc = "OCR2B (rw) register accessor: an alias for `Reg<OCR2B_SPEC>`"]
pub type OCR2B = crate::Reg<ocr2b::OCR2B_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register B Bytes"]
pub mod ocr2b;
#[doc = "TCCR2A (rw) register accessor: an alias for `Reg<TCCR2A_SPEC>`"]
pub type TCCR2A = crate::Reg<tccr2a::TCCR2A_SPEC>;
#[doc = "Timer/Counter2 Control Register A"]
pub mod tccr2a;
#[doc = "TCCR2B (rw) register accessor: an alias for `Reg<TCCR2B_SPEC>`"]
pub type TCCR2B = crate::Reg<tccr2b::TCCR2B_SPEC>;
#[doc = "Timer/Counter2 Control Register B"]
pub mod tccr2b;
#[doc = "TCCR2C (rw) register accessor: an alias for `Reg<TCCR2C_SPEC>`"]
pub type TCCR2C = crate::Reg<tccr2c::TCCR2C_SPEC>;
#[doc = "Timer/Counter2 Control Register C"]
pub mod tccr2c;
#[doc = "TCNT2 (rw) register accessor: an alias for `Reg<TCNT2_SPEC>`"]
pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
#[doc = "Timer/Counter2 Bytes"]
pub mod tcnt2;
#[doc = "TIFR2 (rw) register accessor: an alias for `Reg<TIFR2_SPEC>`"]
pub type TIFR2 = crate::Reg<tifr2::TIFR2_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr2;
#[doc = "TIMSK2 (rw) register accessor: an alias for `Reg<TIMSK2_SPEC>`"]
pub type TIMSK2 = crate::Reg<timsk2::TIMSK2_SPEC>;
#[doc = "Timer/Counter2 Interrupt Mask Register"]
pub mod timsk2;
