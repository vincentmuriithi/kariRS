#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter1 Control Register E"]
    pub tccr1e: TCCR1E,
    _reserved1: [u8; 0x23],
    #[doc = "0x24 - Timer/Counter 1 Dead Time Value"]
    pub dt1: DT1,
    #[doc = "0x25 - Timer/Counter High Bits"]
    pub tc1h: TC1H,
    #[doc = "0x26 - Timer/Counter Control Register D"]
    pub tccr1d: TCCR1D,
    #[doc = "0x27 - Timer/Counter Control Register C"]
    pub tccr1c: TCCR1C,
    _reserved5: [u8; 0x02],
    #[doc = "0x2a - Output compare register"]
    pub ocr1d: OCR1D,
    #[doc = "0x2b - Output compare register"]
    pub ocr1c: OCR1C,
    #[doc = "0x2c - Output Compare Register"]
    pub ocr1b: OCR1B,
    #[doc = "0x2d - Output Compare Register"]
    pub ocr1a: OCR1A,
    #[doc = "0x2e - Timer/Counter Register"]
    pub tcnt1: TCNT1,
    #[doc = "0x2f - Timer/Counter Control Register B"]
    pub tccr1b: TCCR1B,
    #[doc = "0x30 - Timer/Counter Control Register A"]
    pub tccr1a: TCCR1A,
    _reserved12: [u8; 0x07],
    #[doc = "0x38 - Timer/Counter Interrupt Flag Register"]
    pub tifr: TIFR,
    #[doc = "0x39 - Timer/Counter Interrupt Mask Register"]
    pub timsk: TIMSK,
}
#[doc = "DT1 (rw) register accessor: an alias for `Reg<DT1_SPEC>`"]
pub type DT1 = crate::Reg<dt1::DT1_SPEC>;
#[doc = "Timer/Counter 1 Dead Time Value"]
pub mod dt1;
#[doc = "OCR1A (rw) register accessor: an alias for `Reg<OCR1A_SPEC>`"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: an alias for `Reg<OCR1B_SPEC>`"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1b;
#[doc = "OCR1C (rw) register accessor: an alias for `Reg<OCR1C_SPEC>`"]
pub type OCR1C = crate::Reg<ocr1c::OCR1C_SPEC>;
#[doc = "Output compare register"]
pub mod ocr1c;
#[doc = "OCR1D (rw) register accessor: an alias for `Reg<OCR1D_SPEC>`"]
pub type OCR1D = crate::Reg<ocr1d::OCR1D_SPEC>;
#[doc = "Output compare register"]
pub mod ocr1d;
#[doc = "TC1H (rw) register accessor: an alias for `Reg<TC1H_SPEC>`"]
pub type TC1H = crate::Reg<tc1h::TC1H_SPEC>;
#[doc = "Timer/Counter High Bits"]
pub mod tc1h;
#[doc = "TCCR1A (rw) register accessor: an alias for `Reg<TCCR1A_SPEC>`"]
pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
#[doc = "Timer/Counter Control Register A"]
pub mod tccr1a;
#[doc = "TCCR1B (rw) register accessor: an alias for `Reg<TCCR1B_SPEC>`"]
pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
#[doc = "Timer/Counter Control Register B"]
pub mod tccr1b;
#[doc = "TCCR1C (rw) register accessor: an alias for `Reg<TCCR1C_SPEC>`"]
pub type TCCR1C = crate::Reg<tccr1c::TCCR1C_SPEC>;
#[doc = "Timer/Counter Control Register C"]
pub mod tccr1c;
#[doc = "TCCR1D (rw) register accessor: an alias for `Reg<TCCR1D_SPEC>`"]
pub type TCCR1D = crate::Reg<tccr1d::TCCR1D_SPEC>;
#[doc = "Timer/Counter Control Register D"]
pub mod tccr1d;
#[doc = "TCCR1E (rw) register accessor: an alias for `Reg<TCCR1E_SPEC>`"]
pub type TCCR1E = crate::Reg<tccr1e::TCCR1E_SPEC>;
#[doc = "Timer/Counter1 Control Register E"]
pub mod tccr1e;
#[doc = "TCNT1 (rw) register accessor: an alias for `Reg<TCNT1_SPEC>`"]
pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt1;
#[doc = "TIFR (rw) register accessor: an alias for `Reg<TIFR_SPEC>`"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: an alias for `Reg<TIMSK_SPEC>`"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
