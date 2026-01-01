#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer/Counter4 Interrupt Flag register"]
    pub tifr4: TIFR4,
    _reserved1: [u8; 0x38],
    #[doc = "0x39 - Timer/Counter4 Interrupt Mask Register"]
    pub timsk4: TIMSK4,
    _reserved2: [u8; 0x4b],
    #[doc = "0x85 - Timer/Counter4 Low Bytes"]
    pub tcnt4: TCNT4,
    #[doc = "0x86 - Timer/Counter High Bits"]
    pub tc4h: TC4H,
    #[doc = "0x87 - Timer/Counter4 Control Register A"]
    pub tccr4a: TCCR4A,
    #[doc = "0x88 - Timer/Counter4 Control Register B"]
    pub tccr4b: TCCR4B,
    #[doc = "0x89 - Timer/Counter 4 Control Register C"]
    pub tccr4c: TCCR4C,
    #[doc = "0x8a - Timer/Counter 4 Control Register D"]
    pub tccr4d: TCCR4D,
    #[doc = "0x8b - Timer/Counter 4 Control Register E"]
    pub tccr4e: TCCR4E,
    _reserved9: [u8; 0x0a],
    #[doc = "0x96 - Timer/Counter4 Output Compare Register A"]
    pub ocr4a: OCR4A,
    #[doc = "0x97 - Timer/Counter4 Output Compare Register B"]
    pub ocr4b: OCR4B,
    #[doc = "0x98 - Timer/Counter4 Output Compare Register C"]
    pub ocr4c: OCR4C,
    #[doc = "0x99 - Timer/Counter4 Output Compare Register D"]
    pub ocr4d: OCR4D,
    _reserved13: [u8; 0x01],
    #[doc = "0x9b - Timer/Counter 4 Dead Time Value"]
    pub dt4: DT4,
}
#[doc = "DT4 (rw) register accessor: an alias for `Reg<DT4_SPEC>`"]
pub type DT4 = crate::Reg<dt4::DT4_SPEC>;
#[doc = "Timer/Counter 4 Dead Time Value"]
pub mod dt4;
#[doc = "OCR4A (rw) register accessor: an alias for `Reg<OCR4A_SPEC>`"]
pub type OCR4A = crate::Reg<ocr4a::OCR4A_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register A"]
pub mod ocr4a;
#[doc = "OCR4B (rw) register accessor: an alias for `Reg<OCR4B_SPEC>`"]
pub type OCR4B = crate::Reg<ocr4b::OCR4B_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register B"]
pub mod ocr4b;
#[doc = "OCR4C (rw) register accessor: an alias for `Reg<OCR4C_SPEC>`"]
pub type OCR4C = crate::Reg<ocr4c::OCR4C_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register C"]
pub mod ocr4c;
#[doc = "OCR4D (rw) register accessor: an alias for `Reg<OCR4D_SPEC>`"]
pub type OCR4D = crate::Reg<ocr4d::OCR4D_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register D"]
pub mod ocr4d;
#[doc = "TC4H (rw) register accessor: an alias for `Reg<TC4H_SPEC>`"]
pub type TC4H = crate::Reg<tc4h::TC4H_SPEC>;
#[doc = "Timer/Counter High Bits"]
pub mod tc4h;
#[doc = "TCCR4A (rw) register accessor: an alias for `Reg<TCCR4A_SPEC>`"]
pub type TCCR4A = crate::Reg<tccr4a::TCCR4A_SPEC>;
#[doc = "Timer/Counter4 Control Register A"]
pub mod tccr4a;
#[doc = "TCCR4B (rw) register accessor: an alias for `Reg<TCCR4B_SPEC>`"]
pub type TCCR4B = crate::Reg<tccr4b::TCCR4B_SPEC>;
#[doc = "Timer/Counter4 Control Register B"]
pub mod tccr4b;
#[doc = "TCCR4C (rw) register accessor: an alias for `Reg<TCCR4C_SPEC>`"]
pub type TCCR4C = crate::Reg<tccr4c::TCCR4C_SPEC>;
#[doc = "Timer/Counter 4 Control Register C"]
pub mod tccr4c;
#[doc = "TCCR4D (rw) register accessor: an alias for `Reg<TCCR4D_SPEC>`"]
pub type TCCR4D = crate::Reg<tccr4d::TCCR4D_SPEC>;
#[doc = "Timer/Counter 4 Control Register D"]
pub mod tccr4d;
#[doc = "TCCR4E (rw) register accessor: an alias for `Reg<TCCR4E_SPEC>`"]
pub type TCCR4E = crate::Reg<tccr4e::TCCR4E_SPEC>;
#[doc = "Timer/Counter 4 Control Register E"]
pub mod tccr4e;
#[doc = "TCNT4 (rw) register accessor: an alias for `Reg<TCNT4_SPEC>`"]
pub type TCNT4 = crate::Reg<tcnt4::TCNT4_SPEC>;
#[doc = "Timer/Counter4 Low Bytes"]
pub mod tcnt4;
#[doc = "TIFR4 (rw) register accessor: an alias for `Reg<TIFR4_SPEC>`"]
pub type TIFR4 = crate::Reg<tifr4::TIFR4_SPEC>;
#[doc = "Timer/Counter4 Interrupt Flag register"]
pub mod tifr4;
#[doc = "TIMSK4 (rw) register accessor: an alias for `Reg<TIMSK4_SPEC>`"]
pub type TIMSK4 = crate::Reg<timsk4::TIMSK4_SPEC>;
#[doc = "Timer/Counter4 Interrupt Mask Register"]
pub mod timsk4;
