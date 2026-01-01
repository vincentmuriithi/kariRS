#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Strobe"]
    pub strobe: STROBE,
    _reserved1: [u8; 0x0f],
    #[doc = "0x10 - Multiplexer Channel 0"]
    pub channel0: CHANNEL0,
    #[doc = "0x11 - Multiplexer Channel 1"]
    pub channel1: CHANNEL1,
    #[doc = "0x12 - Multiplexer Channel 2"]
    pub channel2: CHANNEL2,
    #[doc = "0x13 - Multiplexer Channel 3"]
    pub channel3: CHANNEL3,
    #[doc = "0x14 - Multiplexer Channel 4"]
    pub channel4: CHANNEL4,
    #[doc = "0x15 - Multiplexer Channel 5"]
    pub channel5: CHANNEL5,
    _reserved7: [u8; 0x0a],
    #[doc = "0x20 - User CCL LUT0 Event A"]
    pub userccllut0a: USERCCLLUT0A,
    #[doc = "0x21 - User CCL LUT0 Event B"]
    pub userccllut0b: USERCCLLUT0B,
    #[doc = "0x22 - User CCL LUT1 Event A"]
    pub userccllut1a: USERCCLLUT1A,
    #[doc = "0x23 - User CCL LUT1 Event B"]
    pub userccllut1b: USERCCLLUT1B,
    #[doc = "0x24 - User CCL LUT2 Event A"]
    pub userccllut2a: USERCCLLUT2A,
    #[doc = "0x25 - User CCL LUT2 Event B"]
    pub userccllut2b: USERCCLLUT2B,
    #[doc = "0x26 - User CCL LUT3 Event A"]
    pub userccllut3a: USERCCLLUT3A,
    #[doc = "0x27 - User CCL LUT3 Event B"]
    pub userccllut3b: USERCCLLUT3B,
    #[doc = "0x28 - User ADC0"]
    pub useradc0: USERADC0,
    #[doc = "0x29 - User EVOUT Port A"]
    pub userevouta: USEREVOUTA,
    #[doc = "0x2a - User EVOUT Port B"]
    pub userevoutb: USEREVOUTB,
    #[doc = "0x2b - User EVOUT Port C"]
    pub userevoutc: USEREVOUTC,
    #[doc = "0x2c - User EVOUT Port D"]
    pub userevoutd: USEREVOUTD,
    #[doc = "0x2d - User EVOUT Port E"]
    pub userevoute: USEREVOUTE,
    #[doc = "0x2e - User EVOUT Port F"]
    pub userevoutf: USEREVOUTF,
    #[doc = "0x2f - User USART0"]
    pub userusart0: USERUSART0,
    #[doc = "0x30 - User USART1"]
    pub userusart1: USERUSART1,
    #[doc = "0x31 - User USART2"]
    pub userusart2: USERUSART2,
    #[doc = "0x32 - User USART3"]
    pub userusart3: USERUSART3,
    #[doc = "0x33 - User TCA0"]
    pub usertca0: USERTCA0,
    #[doc = "0x34 - User TCB0"]
    pub usertcb0: USERTCB0,
    #[doc = "0x35 - User TCB1"]
    pub usertcb1: USERTCB1,
    #[doc = "0x36 - User TCB2"]
    pub usertcb2: USERTCB2,
    #[doc = "0x37 - User TCB3"]
    pub usertcb3: USERTCB3,
}
#[doc = "CHANNEL0 (rw) register accessor: an alias for `Reg<CHANNEL0_SPEC>`"]
pub type CHANNEL0 = crate::Reg<channel0::CHANNEL0_SPEC>;
#[doc = "Multiplexer Channel 0"]
pub mod channel0;
#[doc = "CHANNEL1 (rw) register accessor: an alias for `Reg<CHANNEL1_SPEC>`"]
pub type CHANNEL1 = crate::Reg<channel1::CHANNEL1_SPEC>;
#[doc = "Multiplexer Channel 1"]
pub mod channel1;
#[doc = "CHANNEL2 (rw) register accessor: an alias for `Reg<CHANNEL2_SPEC>`"]
pub type CHANNEL2 = crate::Reg<channel2::CHANNEL2_SPEC>;
#[doc = "Multiplexer Channel 2"]
pub mod channel2;
#[doc = "CHANNEL3 (rw) register accessor: an alias for `Reg<CHANNEL3_SPEC>`"]
pub type CHANNEL3 = crate::Reg<channel3::CHANNEL3_SPEC>;
#[doc = "Multiplexer Channel 3"]
pub mod channel3;
#[doc = "CHANNEL4 (rw) register accessor: an alias for `Reg<CHANNEL4_SPEC>`"]
pub type CHANNEL4 = crate::Reg<channel4::CHANNEL4_SPEC>;
#[doc = "Multiplexer Channel 4"]
pub mod channel4;
#[doc = "CHANNEL5 (rw) register accessor: an alias for `Reg<CHANNEL5_SPEC>`"]
pub type CHANNEL5 = crate::Reg<channel5::CHANNEL5_SPEC>;
#[doc = "Multiplexer Channel 5"]
pub mod channel5;
#[doc = "STROBE (w) register accessor: an alias for `Reg<STROBE_SPEC>`"]
pub type STROBE = crate::Reg<strobe::STROBE_SPEC>;
#[doc = "Channel Strobe"]
pub mod strobe;
#[doc = "USERADC0 (rw) register accessor: an alias for `Reg<USERADC0_SPEC>`"]
pub type USERADC0 = crate::Reg<useradc0::USERADC0_SPEC>;
#[doc = "User ADC0"]
pub mod useradc0;
#[doc = "USERCCLLUT0A (rw) register accessor: an alias for `Reg<USERCCLLUT0A_SPEC>`"]
pub type USERCCLLUT0A = crate::Reg<userccllut0a::USERCCLLUT0A_SPEC>;
#[doc = "User CCL LUT0 Event A"]
pub mod userccllut0a;
#[doc = "USERCCLLUT0B (rw) register accessor: an alias for `Reg<USERCCLLUT0B_SPEC>`"]
pub type USERCCLLUT0B = crate::Reg<userccllut0b::USERCCLLUT0B_SPEC>;
#[doc = "User CCL LUT0 Event B"]
pub mod userccllut0b;
#[doc = "USERCCLLUT1A (rw) register accessor: an alias for `Reg<USERCCLLUT1A_SPEC>`"]
pub type USERCCLLUT1A = crate::Reg<userccllut1a::USERCCLLUT1A_SPEC>;
#[doc = "User CCL LUT1 Event A"]
pub mod userccllut1a;
#[doc = "USERCCLLUT1B (rw) register accessor: an alias for `Reg<USERCCLLUT1B_SPEC>`"]
pub type USERCCLLUT1B = crate::Reg<userccllut1b::USERCCLLUT1B_SPEC>;
#[doc = "User CCL LUT1 Event B"]
pub mod userccllut1b;
#[doc = "USERCCLLUT2A (rw) register accessor: an alias for `Reg<USERCCLLUT2A_SPEC>`"]
pub type USERCCLLUT2A = crate::Reg<userccllut2a::USERCCLLUT2A_SPEC>;
#[doc = "User CCL LUT2 Event A"]
pub mod userccllut2a;
#[doc = "USERCCLLUT2B (rw) register accessor: an alias for `Reg<USERCCLLUT2B_SPEC>`"]
pub type USERCCLLUT2B = crate::Reg<userccllut2b::USERCCLLUT2B_SPEC>;
#[doc = "User CCL LUT2 Event B"]
pub mod userccllut2b;
#[doc = "USERCCLLUT3A (rw) register accessor: an alias for `Reg<USERCCLLUT3A_SPEC>`"]
pub type USERCCLLUT3A = crate::Reg<userccllut3a::USERCCLLUT3A_SPEC>;
#[doc = "User CCL LUT3 Event A"]
pub mod userccllut3a;
#[doc = "USERCCLLUT3B (rw) register accessor: an alias for `Reg<USERCCLLUT3B_SPEC>`"]
pub type USERCCLLUT3B = crate::Reg<userccllut3b::USERCCLLUT3B_SPEC>;
#[doc = "User CCL LUT3 Event B"]
pub mod userccllut3b;
#[doc = "USEREVOUTA (rw) register accessor: an alias for `Reg<USEREVOUTA_SPEC>`"]
pub type USEREVOUTA = crate::Reg<userevouta::USEREVOUTA_SPEC>;
#[doc = "User EVOUT Port A"]
pub mod userevouta;
#[doc = "USEREVOUTB (rw) register accessor: an alias for `Reg<USEREVOUTB_SPEC>`"]
pub type USEREVOUTB = crate::Reg<userevoutb::USEREVOUTB_SPEC>;
#[doc = "User EVOUT Port B"]
pub mod userevoutb;
#[doc = "USEREVOUTC (rw) register accessor: an alias for `Reg<USEREVOUTC_SPEC>`"]
pub type USEREVOUTC = crate::Reg<userevoutc::USEREVOUTC_SPEC>;
#[doc = "User EVOUT Port C"]
pub mod userevoutc;
#[doc = "USEREVOUTD (rw) register accessor: an alias for `Reg<USEREVOUTD_SPEC>`"]
pub type USEREVOUTD = crate::Reg<userevoutd::USEREVOUTD_SPEC>;
#[doc = "User EVOUT Port D"]
pub mod userevoutd;
#[doc = "USEREVOUTE (rw) register accessor: an alias for `Reg<USEREVOUTE_SPEC>`"]
pub type USEREVOUTE = crate::Reg<userevoute::USEREVOUTE_SPEC>;
#[doc = "User EVOUT Port E"]
pub mod userevoute;
#[doc = "USEREVOUTF (rw) register accessor: an alias for `Reg<USEREVOUTF_SPEC>`"]
pub type USEREVOUTF = crate::Reg<userevoutf::USEREVOUTF_SPEC>;
#[doc = "User EVOUT Port F"]
pub mod userevoutf;
#[doc = "USERTCA0 (rw) register accessor: an alias for `Reg<USERTCA0_SPEC>`"]
pub type USERTCA0 = crate::Reg<usertca0::USERTCA0_SPEC>;
#[doc = "User TCA0"]
pub mod usertca0;
#[doc = "USERTCB0 (rw) register accessor: an alias for `Reg<USERTCB0_SPEC>`"]
pub type USERTCB0 = crate::Reg<usertcb0::USERTCB0_SPEC>;
#[doc = "User TCB0"]
pub mod usertcb0;
#[doc = "USERTCB1 (rw) register accessor: an alias for `Reg<USERTCB1_SPEC>`"]
pub type USERTCB1 = crate::Reg<usertcb1::USERTCB1_SPEC>;
#[doc = "User TCB1"]
pub mod usertcb1;
#[doc = "USERTCB2 (rw) register accessor: an alias for `Reg<USERTCB2_SPEC>`"]
pub type USERTCB2 = crate::Reg<usertcb2::USERTCB2_SPEC>;
#[doc = "User TCB2"]
pub mod usertcb2;
#[doc = "USERTCB3 (rw) register accessor: an alias for `Reg<USERTCB3_SPEC>`"]
pub type USERTCB3 = crate::Reg<usertcb3::USERTCB3_SPEC>;
#[doc = "User TCB3"]
pub mod usertcb3;
#[doc = "USERUSART0 (rw) register accessor: an alias for `Reg<USERUSART0_SPEC>`"]
pub type USERUSART0 = crate::Reg<userusart0::USERUSART0_SPEC>;
#[doc = "User USART0"]
pub mod userusart0;
#[doc = "USERUSART1 (rw) register accessor: an alias for `Reg<USERUSART1_SPEC>`"]
pub type USERUSART1 = crate::Reg<userusart1::USERUSART1_SPEC>;
#[doc = "User USART1"]
pub mod userusart1;
#[doc = "USERUSART2 (rw) register accessor: an alias for `Reg<USERUSART2_SPEC>`"]
pub type USERUSART2 = crate::Reg<userusart2::USERUSART2_SPEC>;
#[doc = "User USART2"]
pub mod userusart2;
#[doc = "USERUSART3 (rw) register accessor: an alias for `Reg<USERUSART3_SPEC>`"]
pub type USERUSART3 = crate::Reg<userusart3::USERUSART3_SPEC>;
#[doc = "User USART3"]
pub mod userusart3;
