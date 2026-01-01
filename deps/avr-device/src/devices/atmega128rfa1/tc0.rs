#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter0 Interrupt Flag Register"]
    pub tifr0: TIFR0,
    _reserved1: [u8; 0x0d],
    #[doc = "0x0e - General Timer/Counter Control Register"]
    pub gtccr: GTCCR,
    #[doc = "0x0f - Timer/Counter0 Control Register A"]
    pub tccr0a: TCCR0A,
    #[doc = "0x10 - Timer/Counter0 Control Register B"]
    pub tccr0b: TCCR0B,
    #[doc = "0x11 - Timer/Counter0 Register"]
    pub tcnt0: TCNT0,
    #[doc = "0x12 - Timer/Counter0 Output Compare Register"]
    pub ocr0a: OCR0A,
    #[doc = "0x13 - Timer/Counter0 Output Compare Register B"]
    pub ocr0b: OCR0B,
    _reserved7: [u8; 0x25],
    #[doc = "0x39 - Timer/Counter0 Interrupt Mask Register"]
    pub timsk0: TIMSK0,
}
#[doc = "GTCCR (rw) register accessor: an alias for `Reg<GTCCR_SPEC>`"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer/Counter Control Register"]
pub mod gtccr;
#[doc = "OCR0A (rw) register accessor: an alias for `Reg<OCR0A_SPEC>`"]
pub type OCR0A = crate::Reg<ocr0a::OCR0A_SPEC>;
#[doc = "Timer/Counter0 Output Compare Register"]
pub mod ocr0a;
#[doc = "OCR0B (rw) register accessor: an alias for `Reg<OCR0B_SPEC>`"]
pub type OCR0B = crate::Reg<ocr0b::OCR0B_SPEC>;
#[doc = "Timer/Counter0 Output Compare Register B"]
pub mod ocr0b;
#[doc = "TCCR0A (rw) register accessor: an alias for `Reg<TCCR0A_SPEC>`"]
pub type TCCR0A = crate::Reg<tccr0a::TCCR0A_SPEC>;
#[doc = "Timer/Counter0 Control Register A"]
pub mod tccr0a;
#[doc = "TCCR0B (rw) register accessor: an alias for `Reg<TCCR0B_SPEC>`"]
pub type TCCR0B = crate::Reg<tccr0b::TCCR0B_SPEC>;
#[doc = "Timer/Counter0 Control Register B"]
pub mod tccr0b;
#[doc = "TCNT0 (rw) register accessor: an alias for `Reg<TCNT0_SPEC>`"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter0 Register"]
pub mod tcnt0;
#[doc = "TIFR0 (rw) register accessor: an alias for `Reg<TIFR0_SPEC>`"]
pub type TIFR0 = crate::Reg<tifr0::TIFR0_SPEC>;
#[doc = "Timer/Counter0 Interrupt Flag Register"]
pub mod tifr0;
#[doc = "TIMSK0 (rw) register accessor: an alias for `Reg<TIMSK0_SPEC>`"]
pub type TIMSK0 = crate::Reg<timsk0::TIMSK0_SPEC>;
#[doc = "Timer/Counter0 Interrupt Mask Register"]
pub mod timsk0;
