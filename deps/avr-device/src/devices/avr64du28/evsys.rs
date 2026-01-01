#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Event A"]
    pub sweventa: SWEVENTA,
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
    #[doc = "0x20 - CCL0 Event A"]
    pub userccllut0a: USERCCLLUT0A,
    #[doc = "0x21 - CCL0 Event B"]
    pub userccllut0b: USERCCLLUT0B,
    #[doc = "0x22 - CCL1 Event A"]
    pub userccllut1a: USERCCLLUT1A,
    #[doc = "0x23 - CCL1 Event B"]
    pub userccllut1b: USERCCLLUT1B,
    #[doc = "0x24 - CCL2 Event A"]
    pub userccllut2a: USERCCLLUT2A,
    #[doc = "0x25 - CCL2 Event B"]
    pub userccllut2b: USERCCLLUT2B,
    #[doc = "0x26 - CCL3 Event A"]
    pub userccllut3a: USERCCLLUT3A,
    #[doc = "0x27 - CCL3 Event B"]
    pub userccllut3b: USERCCLLUT3B,
    #[doc = "0x28 - ADC0"]
    pub useradc0start: USERADC0START,
    #[doc = "0x29 - EVOUTA"]
    pub userevsysevouta: USEREVSYSEVOUTA,
    #[doc = "0x2a - EVOUTD"]
    pub userevsysevoutd: USEREVSYSEVOUTD,
    #[doc = "0x2b - EVOUTF"]
    pub userevsysevoutf: USEREVSYSEVOUTF,
    #[doc = "0x2c - USART0"]
    pub userusart0irda: USERUSART0IRDA,
    #[doc = "0x2d - USART1"]
    pub userusart1irda: USERUSART1IRDA,
    #[doc = "0x2e - TCA0 Event A"]
    pub usertca0cnta: USERTCA0CNTA,
    #[doc = "0x2f - TCA0 Event B"]
    pub usertca0cntb: USERTCA0CNTB,
    #[doc = "0x30 - TCB0 Event A"]
    pub usertcb0capt: USERTCB0CAPT,
    #[doc = "0x31 - TCB0 Event B"]
    pub usertcb0count: USERTCB0COUNT,
    #[doc = "0x32 - TCB1 Event A"]
    pub usertcb1capt: USERTCB1CAPT,
    #[doc = "0x33 - TCB1 Event B"]
    pub usertcb1count: USERTCB1COUNT,
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
#[doc = "SWEVENTA (rw) register accessor: an alias for `Reg<SWEVENTA_SPEC>`"]
pub type SWEVENTA = crate::Reg<sweventa::SWEVENTA_SPEC>;
#[doc = "Software Event A"]
pub mod sweventa;
#[doc = "USERADC0START (rw) register accessor: an alias for `Reg<USERADC0START_SPEC>`"]
pub type USERADC0START = crate::Reg<useradc0start::USERADC0START_SPEC>;
#[doc = "ADC0"]
pub mod useradc0start;
#[doc = "USERCCLLUT0A (rw) register accessor: an alias for `Reg<USERCCLLUT0A_SPEC>`"]
pub type USERCCLLUT0A = crate::Reg<userccllut0a::USERCCLLUT0A_SPEC>;
#[doc = "CCL0 Event A"]
pub mod userccllut0a;
#[doc = "USERCCLLUT0B (rw) register accessor: an alias for `Reg<USERCCLLUT0B_SPEC>`"]
pub type USERCCLLUT0B = crate::Reg<userccllut0b::USERCCLLUT0B_SPEC>;
#[doc = "CCL0 Event B"]
pub mod userccllut0b;
#[doc = "USERCCLLUT1A (rw) register accessor: an alias for `Reg<USERCCLLUT1A_SPEC>`"]
pub type USERCCLLUT1A = crate::Reg<userccllut1a::USERCCLLUT1A_SPEC>;
#[doc = "CCL1 Event A"]
pub mod userccllut1a;
#[doc = "USERCCLLUT1B (rw) register accessor: an alias for `Reg<USERCCLLUT1B_SPEC>`"]
pub type USERCCLLUT1B = crate::Reg<userccllut1b::USERCCLLUT1B_SPEC>;
#[doc = "CCL1 Event B"]
pub mod userccllut1b;
#[doc = "USERCCLLUT2A (rw) register accessor: an alias for `Reg<USERCCLLUT2A_SPEC>`"]
pub type USERCCLLUT2A = crate::Reg<userccllut2a::USERCCLLUT2A_SPEC>;
#[doc = "CCL2 Event A"]
pub mod userccllut2a;
#[doc = "USERCCLLUT2B (rw) register accessor: an alias for `Reg<USERCCLLUT2B_SPEC>`"]
pub type USERCCLLUT2B = crate::Reg<userccllut2b::USERCCLLUT2B_SPEC>;
#[doc = "CCL2 Event B"]
pub mod userccllut2b;
#[doc = "USERCCLLUT3A (rw) register accessor: an alias for `Reg<USERCCLLUT3A_SPEC>`"]
pub type USERCCLLUT3A = crate::Reg<userccllut3a::USERCCLLUT3A_SPEC>;
#[doc = "CCL3 Event A"]
pub mod userccllut3a;
#[doc = "USERCCLLUT3B (rw) register accessor: an alias for `Reg<USERCCLLUT3B_SPEC>`"]
pub type USERCCLLUT3B = crate::Reg<userccllut3b::USERCCLLUT3B_SPEC>;
#[doc = "CCL3 Event B"]
pub mod userccllut3b;
#[doc = "USEREVSYSEVOUTA (rw) register accessor: an alias for `Reg<USEREVSYSEVOUTA_SPEC>`"]
pub type USEREVSYSEVOUTA = crate::Reg<userevsysevouta::USEREVSYSEVOUTA_SPEC>;
#[doc = "EVOUTA"]
pub mod userevsysevouta;
#[doc = "USEREVSYSEVOUTD (rw) register accessor: an alias for `Reg<USEREVSYSEVOUTD_SPEC>`"]
pub type USEREVSYSEVOUTD = crate::Reg<userevsysevoutd::USEREVSYSEVOUTD_SPEC>;
#[doc = "EVOUTD"]
pub mod userevsysevoutd;
#[doc = "USEREVSYSEVOUTF (rw) register accessor: an alias for `Reg<USEREVSYSEVOUTF_SPEC>`"]
pub type USEREVSYSEVOUTF = crate::Reg<userevsysevoutf::USEREVSYSEVOUTF_SPEC>;
#[doc = "EVOUTF"]
pub mod userevsysevoutf;
#[doc = "USERTCA0CNTA (rw) register accessor: an alias for `Reg<USERTCA0CNTA_SPEC>`"]
pub type USERTCA0CNTA = crate::Reg<usertca0cnta::USERTCA0CNTA_SPEC>;
#[doc = "TCA0 Event A"]
pub mod usertca0cnta;
#[doc = "USERTCA0CNTB (rw) register accessor: an alias for `Reg<USERTCA0CNTB_SPEC>`"]
pub type USERTCA0CNTB = crate::Reg<usertca0cntb::USERTCA0CNTB_SPEC>;
#[doc = "TCA0 Event B"]
pub mod usertca0cntb;
#[doc = "USERTCB0CAPT (rw) register accessor: an alias for `Reg<USERTCB0CAPT_SPEC>`"]
pub type USERTCB0CAPT = crate::Reg<usertcb0capt::USERTCB0CAPT_SPEC>;
#[doc = "TCB0 Event A"]
pub mod usertcb0capt;
#[doc = "USERTCB0COUNT (rw) register accessor: an alias for `Reg<USERTCB0COUNT_SPEC>`"]
pub type USERTCB0COUNT = crate::Reg<usertcb0count::USERTCB0COUNT_SPEC>;
#[doc = "TCB0 Event B"]
pub mod usertcb0count;
#[doc = "USERTCB1CAPT (rw) register accessor: an alias for `Reg<USERTCB1CAPT_SPEC>`"]
pub type USERTCB1CAPT = crate::Reg<usertcb1capt::USERTCB1CAPT_SPEC>;
#[doc = "TCB1 Event A"]
pub mod usertcb1capt;
#[doc = "USERTCB1COUNT (rw) register accessor: an alias for `Reg<USERTCB1COUNT_SPEC>`"]
pub type USERTCB1COUNT = crate::Reg<usertcb1count::USERTCB1COUNT_SPEC>;
#[doc = "TCB1 Event B"]
pub mod usertcb1count;
#[doc = "USERUSART0IRDA (rw) register accessor: an alias for `Reg<USERUSART0IRDA_SPEC>`"]
pub type USERUSART0IRDA = crate::Reg<userusart0irda::USERUSART0IRDA_SPEC>;
#[doc = "USART0"]
pub mod userusart0irda;
#[doc = "USERUSART1IRDA (rw) register accessor: an alias for `Reg<USERUSART1IRDA_SPEC>`"]
pub type USERUSART1IRDA = crate::Reg<userusart1irda::USERUSART1IRDA_SPEC>;
#[doc = "USART1"]
pub mod userusart1irda;
