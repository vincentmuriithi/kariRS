#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART0 MSPIM Control and Status Register A"]
    pub ucsr0a: UCSR0A,
    #[doc = "0x01 - USART0 MSPIM Control and Status Register B"]
    pub ucsr0b: UCSR0B,
    #[doc = "0x02 - USART0 MSPIM Control and Status Register C"]
    pub ucsr0c: UCSR0C,
}
#[doc = "UCSR0A (rw) register accessor: an alias for `Reg<UCSR0A_SPEC>`"]
pub type UCSR0A = crate::Reg<ucsr0a::UCSR0A_SPEC>;
#[doc = "USART0 MSPIM Control and Status Register A"]
pub mod ucsr0a;
#[doc = "UCSR0B (rw) register accessor: an alias for `Reg<UCSR0B_SPEC>`"]
pub type UCSR0B = crate::Reg<ucsr0b::UCSR0B_SPEC>;
#[doc = "USART0 MSPIM Control and Status Register B"]
pub mod ucsr0b;
#[doc = "UCSR0C (rw) register accessor: an alias for `Reg<UCSR0C_SPEC>`"]
pub type UCSR0C = crate::Reg<ucsr0c::UCSR0C_SPEC>;
#[doc = "USART0 MSPIM Control and Status Register C"]
pub mod ucsr0c;
