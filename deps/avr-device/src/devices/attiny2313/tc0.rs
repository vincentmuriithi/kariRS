#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter Control Register A"]
    pub tccr0a: TCCR0A,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Timer/Counter0"]
    pub tcnt0: TCNT0,
    #[doc = "0x03 - Timer/Counter Control Register B"]
    pub tccr0b: TCCR0B,
    _reserved3: [u8; 0x02],
    #[doc = "0x06 - Timer/Counter0 Output Compare Register"]
    pub ocr0a: OCR0A,
    _reserved4: [u8; 0x01],
    #[doc = "0x08 - Timer/Counter Interrupt Flag register"]
    pub tifr: TIFR,
    #[doc = "0x09 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
    _reserved6: [u8; 0x02],
    #[doc = "0x0c - Timer/Counter0 Output Compare Register"]
    pub ocr0b: OCR0B,
}
#[doc = "OCR0A (rw) register accessor: an alias for `Reg<OCR0A_SPEC>`"]
pub type OCR0A = crate::Reg<ocr0a::OCR0A_SPEC>;
#[doc = "Timer/Counter0 Output Compare Register"]
pub mod ocr0a;
#[doc = "OCR0B (rw) register accessor: an alias for `Reg<OCR0B_SPEC>`"]
pub type OCR0B = crate::Reg<ocr0b::OCR0B_SPEC>;
#[doc = "Timer/Counter0 Output Compare Register"]
pub mod ocr0b;
#[doc = "TCCR0A (rw) register accessor: an alias for `Reg<TCCR0A_SPEC>`"]
pub type TCCR0A = crate::Reg<tccr0a::TCCR0A_SPEC>;
#[doc = "Timer/Counter Control Register A"]
pub mod tccr0a;
#[doc = "TCCR0B (rw) register accessor: an alias for `Reg<TCCR0B_SPEC>`"]
pub type TCCR0B = crate::Reg<tccr0b::TCCR0B_SPEC>;
#[doc = "Timer/Counter Control Register B"]
pub mod tccr0b;
#[doc = "TCNT0 (rw) register accessor: an alias for `Reg<TCNT0_SPEC>`"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter0"]
pub mod tcnt0;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
