#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Baud Rate Register Hight Byte"]
    pub ubrr1h: UBRR1H,
    #[doc = "0x01 - USART Baud Rate Register Low Byte"]
    pub ubrr1l: UBRR1L,
    #[doc = "0x02 - USART Control and Status Register B"]
    pub ucsr1b: UCSR1B,
    #[doc = "0x03 - USART Control and Status Register A"]
    pub ucsr1a: UCSR1A,
    #[doc = "0x04 - USART I/O Data Register"]
    pub udr1: UDR1,
    #[doc = "0x05 - USART Control and Status Register C"]
    pub ucsr1c: UCSR1C,
}
#[doc = "UBRR1H (rw) register accessor: an alias for `Reg<UBRR1H_SPEC>`"]
pub type UBRR1H = crate::Reg<ubrr1h::UBRR1H_SPEC>;
#[doc = "USART Baud Rate Register Hight Byte"]
pub mod ubrr1h;
#[doc = "UBRR1L (rw) register accessor: an alias for `Reg<UBRR1L_SPEC>`"]
pub type UBRR1L = crate::Reg<ubrr1l::UBRR1L_SPEC>;
#[doc = "USART Baud Rate Register Low Byte"]
pub mod ubrr1l;
#[doc = "UCSR1A (rw) register accessor: an alias for `Reg<UCSR1A_SPEC>`"]
pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr1a;
#[doc = "UCSR1B (rw) register accessor: an alias for `Reg<UCSR1B_SPEC>`"]
pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr1b;
#[doc = "UCSR1C (rw) register accessor: an alias for `Reg<UCSR1C_SPEC>`"]
pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr1c;
#[doc = "UDR1 (rw) register accessor: an alias for `Reg<UDR1_SPEC>`"]
pub type UDR1 = crate::Reg<udr1::UDR1_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr1;
