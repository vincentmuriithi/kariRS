#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Baud Rate Register Low Byte"]
    pub ubrr0l: UBRR0L,
    #[doc = "0x01 - USART Control and Status Register B"]
    pub ucsr0b: UCSR0B,
    #[doc = "0x02 - USART Control and Status Register A"]
    pub ucsr0a: UCSR0A,
    #[doc = "0x03 - USART I/O Data Register"]
    pub udr0: UDR0,
    _reserved4: [u8; 0x63],
    #[doc = "0x67 - USART Baud Rate Register Hight Byte"]
    pub ubrr0h: UBRR0H,
    _reserved5: [u8; 0x04],
    #[doc = "0x6c - USART Control and Status Register C"]
    pub ucsr0c: UCSR0C,
}
#[doc = "UBRR0H (rw) register accessor: an alias for `Reg<UBRR0H_SPEC>`"]
pub type UBRR0H = crate::Reg<ubrr0h::UBRR0H_SPEC>;
#[doc = "USART Baud Rate Register Hight Byte"]
pub mod ubrr0h;
#[doc = "UBRR0L (rw) register accessor: an alias for `Reg<UBRR0L_SPEC>`"]
pub type UBRR0L = crate::Reg<ubrr0l::UBRR0L_SPEC>;
#[doc = "USART Baud Rate Register Low Byte"]
pub mod ubrr0l;
#[doc = "UCSR0A (rw) register accessor: an alias for `Reg<UCSR0A_SPEC>`"]
pub type UCSR0A = crate::Reg<ucsr0a::UCSR0A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr0a;
#[doc = "UCSR0B (rw) register accessor: an alias for `Reg<UCSR0B_SPEC>`"]
pub type UCSR0B = crate::Reg<ucsr0b::UCSR0B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr0b;
#[doc = "UCSR0C (rw) register accessor: an alias for `Reg<UCSR0C_SPEC>`"]
pub type UCSR0C = crate::Reg<ucsr0c::UCSR0C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr0c;
#[doc = "UDR0 (rw) register accessor: an alias for `Reg<UDR0_SPEC>`"]
pub type UDR0 = crate::Reg<udr0::UDR0_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr0;
