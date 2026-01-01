#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART1 Control and Status Register A"]
    pub ucsr1a: UCSR1A,
    #[doc = "0x01 - USART1 Control and Status Register B"]
    pub ucsr1b: UCSR1B,
    #[doc = "0x02 - USART1 Control and Status Register C"]
    pub ucsr1c: UCSR1C,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - USART1 Baud Rate Register Bytes"]
    pub ubrr1: UBRR1,
    #[doc = "0x06 - USART1 I/O Data Register"]
    pub udr1: UDR1,
}
#[doc = "UBRR1 (rw) register accessor: an alias for `Reg<UBRR1_SPEC>`"]
pub type UBRR1 = crate::Reg<ubrr1::UBRR1_SPEC>;
#[doc = "USART1 Baud Rate Register Bytes"]
pub mod ubrr1;
#[doc = "UCSR1A (rw) register accessor: an alias for `Reg<UCSR1A_SPEC>`"]
pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
#[doc = "USART1 Control and Status Register A"]
pub mod ucsr1a;
#[doc = "UCSR1B (rw) register accessor: an alias for `Reg<UCSR1B_SPEC>`"]
pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
#[doc = "USART1 Control and Status Register B"]
pub mod ucsr1b;
#[doc = "UCSR1C (rw) register accessor: an alias for `Reg<UCSR1C_SPEC>`"]
pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
#[doc = "USART1 Control and Status Register C"]
pub mod ucsr1c;
#[doc = "UDR1 (rw) register accessor: an alias for `Reg<UDR1_SPEC>`"]
pub type UDR1 = crate::Reg<udr1::UDR1_SPEC>;
#[doc = "USART1 I/O Data Register"]
pub mod udr1;
