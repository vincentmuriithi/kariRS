#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART1 MSPIM Control and Status Register A"]
    pub ucsr1a: UCSR1A,
    #[doc = "0x01 - USART1 MSPIM Control and Status Register B"]
    pub ucsr1b: UCSR1B,
    #[doc = "0x02 - USART1 MSPIM Control and Status Register C"]
    pub ucsr1c: UCSR1C,
}
#[doc = "UCSR1A (rw) register accessor: an alias for `Reg<UCSR1A_SPEC>`"]
pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
#[doc = "USART1 MSPIM Control and Status Register A"]
pub mod ucsr1a;
#[doc = "UCSR1B (rw) register accessor: an alias for `Reg<UCSR1B_SPEC>`"]
pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
#[doc = "USART1 MSPIM Control and Status Register B"]
pub mod ucsr1b;
#[doc = "UCSR1C (rw) register accessor: an alias for `Reg<UCSR1C_SPEC>`"]
pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
#[doc = "USART1 MSPIM Control and Status Register C"]
pub mod ucsr1c;
