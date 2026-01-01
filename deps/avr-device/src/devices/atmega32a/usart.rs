#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Baud Rate Register Low Byte"]
    pub ubrrl: UBRRL,
    #[doc = "0x01 - USART Control and Status Register B"]
    pub ucsrb: UCSRB,
    #[doc = "0x02 - USART Control and Status Register A"]
    pub ucsra: UCSRA,
    #[doc = "0x03 - USART I/O Data Register"]
    pub udr: UDR,
    _reserved4: [u8; 0x13],
    _reserved_4_ubrrh: [u8; 0x01],
}
impl RegisterBlock {
    #[doc = "0x17 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsrc(&self) -> &UCSRC {
        unsafe { &*(self as *const Self).cast::<u8>().add(23usize).cast() }
    }
    #[doc = "0x17 - USART Baud Rate Register Hight Byte"]
    #[inline(always)]
    pub const fn ubrrh(&self) -> &UBRRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(23usize).cast() }
    }
}
#[doc = "UBRRH (rw) register accessor: an alias for `Reg<UBRRH_SPEC>`"]
pub type UBRRH = crate::Reg<ubrrh::UBRRH_SPEC>;
#[doc = "USART Baud Rate Register Hight Byte"]
pub mod ubrrh;
#[doc = "UBRRL (rw) register accessor: an alias for `Reg<UBRRL_SPEC>`"]
pub type UBRRL = crate::Reg<ubrrl::UBRRL_SPEC>;
#[doc = "USART Baud Rate Register Low Byte"]
pub mod ubrrl;
#[doc = "UCSRA (rw) register accessor: an alias for `Reg<UCSRA_SPEC>`"]
pub type UCSRA = crate::Reg<ucsra::UCSRA_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsra;
#[doc = "UCSRB (rw) register accessor: an alias for `Reg<UCSRB_SPEC>`"]
pub type UCSRB = crate::Reg<ucsrb::UCSRB_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsrb;
#[doc = "UCSRC (rw) register accessor: an alias for `Reg<UCSRC_SPEC>`"]
pub type UCSRC = crate::Reg<ucsrc::UCSRC_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsrc;
#[doc = "UDR (rw) register accessor: an alias for `Reg<UDR_SPEC>`"]
pub type UDR = crate::Reg<udr::UDR_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr;
