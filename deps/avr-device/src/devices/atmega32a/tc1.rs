#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter1 Input Capture Register Bytes"]
    pub icr1: ICR1,
    #[doc = "0x02 - Timer/Counter1 Output Compare Register Bytes"]
    pub ocr1b: OCR1B,
    #[doc = "0x04 - Timer/Counter1 Output Compare Register Bytes"]
    pub ocr1a: OCR1A,
    #[doc = "0x06 - Timer/Counter1 Bytes"]
    pub tcnt1: TCNT1,
    #[doc = "0x08 - Timer/Counter1 Control Register B"]
    pub tccr1b: TCCR1B,
    #[doc = "0x09 - Timer/Counter1 Control Register A"]
    pub tccr1a: TCCR1A,
    _reserved6: [u8; 0x08],
    #[doc = "0x12 - Timer/Counter Interrupt Flag register"]
    pub tifr: TIFR,
    #[doc = "0x13 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
}
#[doc = "ICR1 (rw) register accessor: an alias for `Reg<ICR1_SPEC>`"]
pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
#[doc = "Timer/Counter1 Input Capture Register Bytes"]
pub mod icr1;
#[doc = "OCR1A (rw) register accessor: an alias for `Reg<OCR1A_SPEC>`"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Timer/Counter1 Output Compare Register Bytes"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: an alias for `Reg<OCR1B_SPEC>`"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Timer/Counter1 Output Compare Register Bytes"]
pub mod ocr1b;
#[doc = "TCCR1A (rw) register accessor: an alias for `Reg<TCCR1A_SPEC>`"]
pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
#[doc = "Timer/Counter1 Control Register A"]
pub mod tccr1a;
#[doc = "TCCR1B (rw) register accessor: an alias for `Reg<TCCR1B_SPEC>`"]
pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
#[doc = "Timer/Counter1 Control Register B"]
pub mod tccr1b;
#[doc = "TCNT1 (rw) register accessor: an alias for `Reg<TCNT1_SPEC>`"]
pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
#[doc = "Timer/Counter1 Bytes"]
pub mod tcnt1;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
