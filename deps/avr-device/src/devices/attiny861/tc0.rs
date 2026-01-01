#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter0 Output Compare Register"]
    pub ocr0b: OCR0B,
    #[doc = "0x01 - Timer/Counter0 Output Compare Register"]
    pub ocr0a: OCR0A,
    #[doc = "0x02 - Timer/Counter0 High"]
    pub tcnt0h: TCNT0H,
    #[doc = "0x03 - Timer/Counter Control Register A"]
    pub tccr0a: TCCR0A,
    _reserved4: [u8; 0x1c],
    #[doc = "0x20 - Timer/Counter0 Low"]
    pub tcnt0l: TCNT0L,
    #[doc = "0x21 - Timer/Counter Control Register B"]
    pub tccr0b: TCCR0B,
    _reserved6: [u8; 0x04],
    #[doc = "0x26 - Timer/Counter0 Interrupt Flag register"]
    pub tifr: TIFR,
    #[doc = "0x27 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
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
#[doc = "TCNT0H (rw) register accessor: an alias for `Reg<TCNT0H_SPEC>`"]
pub type TCNT0H = crate::Reg<tcnt0h::TCNT0H_SPEC>;
#[doc = "Timer/Counter0 High"]
pub mod tcnt0h;
#[doc = "TCNT0L (rw) register accessor: an alias for `Reg<TCNT0L_SPEC>`"]
pub type TCNT0L = crate::Reg<tcnt0l::TCNT0L_SPEC>;
#[doc = "Timer/Counter0 Low"]
pub mod tcnt0l;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter0 Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
