#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter Interrupt Flag Register"]
    pub tifr2: TIFR2,
    _reserved1: [u8; 0x0b],
    #[doc = "0x0c - General Timer Counter Control register"]
    pub gtccr: GTCCR,
    _reserved2: [u8; 0x2c],
    #[doc = "0x39 - Timer/Counter Interrupt Mask register"]
    pub timsk2: TIMSK2,
    _reserved3: [u8; 0x3f],
    #[doc = "0x79 - Timer/Counter2 Control Register A"]
    pub tccr2a: TCCR2A,
    #[doc = "0x7a - Timer/Counter2 Control Register B"]
    pub tccr2b: TCCR2B,
    #[doc = "0x7b - Timer/Counter2"]
    pub tcnt2: TCNT2,
    #[doc = "0x7c - Timer/Counter2 Output Compare Register A"]
    pub ocr2a: OCR2A,
    #[doc = "0x7d - Timer/Counter2 Output Compare Register B"]
    pub ocr2b: OCR2B,
    _reserved8: [u8; 0x01],
    #[doc = "0x7f - Asynchronous Status Register"]
    pub assr: ASSR,
}
#[doc = "ASSR (rw) register accessor: an alias for `Reg<ASSR_SPEC>`"]
pub type ASSR = crate::Reg<assr::ASSR_SPEC>;
#[doc = "Asynchronous Status Register"]
pub mod assr;
#[doc = "GTCCR (rw) register accessor: an alias for `Reg<GTCCR_SPEC>`"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer Counter Control register"]
pub mod gtccr;
#[doc = "OCR2A (rw) register accessor: an alias for `Reg<OCR2A_SPEC>`"]
pub type OCR2A = crate::Reg<ocr2a::OCR2A_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register A"]
pub mod ocr2a;
#[doc = "OCR2B (rw) register accessor: an alias for `Reg<OCR2B_SPEC>`"]
pub type OCR2B = crate::Reg<ocr2b::OCR2B_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register B"]
pub mod ocr2b;
#[doc = "TCCR2A (rw) register accessor: an alias for `Reg<TCCR2A_SPEC>`"]
pub type TCCR2A = crate::Reg<tccr2a::TCCR2A_SPEC>;
#[doc = "Timer/Counter2 Control Register A"]
pub mod tccr2a;
#[doc = "TCCR2B (rw) register accessor: an alias for `Reg<TCCR2B_SPEC>`"]
pub type TCCR2B = crate::Reg<tccr2b::TCCR2B_SPEC>;
#[doc = "Timer/Counter2 Control Register B"]
pub mod tccr2b;
#[doc = "TCNT2 (rw) register accessor: an alias for `Reg<TCNT2_SPEC>`"]
pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
#[doc = "Timer/Counter2"]
pub mod tcnt2;
#[doc = "TIFR2 (rw) register accessor: an alias for `Reg<TIFR2_SPEC>`"]
pub type TIFR2 = crate::Reg<tifr2::TIFR2_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr2;
#[doc = "TIMSK2 (rw) register accessor: an alias for `Reg<TIMSK2_SPEC>`"]
pub type TIMSK2 = crate::Reg<timsk2::TIMSK2_SPEC>;
#[doc = "Timer/Counter Interrupt Mask register"]
pub mod timsk2;
