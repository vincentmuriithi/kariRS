#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - No Description."]
    CRCSCAN_NMI = 1,
    #[doc = "2 - No Description."]
    BOD_VLM = 2,
    #[doc = "3 - No Description."]
    CLKCTRL_CFD = 3,
    #[doc = "4 - No Description."]
    RTC_CNT = 4,
    #[doc = "5 - No Description."]
    RTC_PIT = 5,
    #[doc = "6 - No Description."]
    CCL_CCL = 6,
    #[doc = "7 - No Description."]
    USB0_BUSEVENT = 7,
    #[doc = "8 - No Description."]
    USB0_TRNCOMPL = 8,
    #[doc = "9 - No Description."]
    PORTA_PORT = 9,
    #[doc = "10 - No Description."]
    TCA0_LUNF_OVF = 10,
    #[doc = "11 - No Description."]
    TCA0_HUNF = 11,
    #[doc = "12 - No Description."]
    TCA0_CMP0_LCMP0 = 12,
    #[doc = "13 - No Description."]
    TCA0_CMP1_LCMP1 = 13,
    #[doc = "14 - No Description."]
    TCA0_CMP2_LCMP2 = 14,
    #[doc = "15 - No Description."]
    TCB0_INT = 15,
    #[doc = "16 - No Description."]
    TWI0_TWIS = 16,
    #[doc = "17 - No Description."]
    TWI0_TWIM = 17,
    #[doc = "18 - No Description."]
    SPI0_INT = 18,
    #[doc = "19 - No Description."]
    USART0_RXC = 19,
    #[doc = "20 - No Description."]
    USART0_DRE = 20,
    #[doc = "21 - No Description."]
    USART0_TXC = 21,
    #[doc = "22 - No Description."]
    PORTD_PORT = 22,
    #[doc = "23 - No Description."]
    PORTC_PORT = 23,
    #[doc = "24 - No Description."]
    PORTF_PORT = 24,
    #[doc = "25 - No Description."]
    NVMCTRL_NVMREADY = 25,
    #[doc = "26 - No Description."]
    USART1_RXC = 26,
    #[doc = "27 - No Description."]
    USART1_DRE = 27,
    #[doc = "28 - No Description."]
    USART1_TXC = 28,
    #[doc = "29 - No Description."]
    TCB1_INT = 29,
    #[doc = "30 - No Description."]
    AC0_AC = 30,
    #[doc = "31 - No Description."]
    ADC0_ERROR = 31,
    #[doc = "32 - No Description."]
    ADC0_RESRDY = 32,
    #[doc = "33 - No Description."]
    ADC0_SAMPRDY = 33,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::CRCSCAN_NMI),
            2 => Ok(Interrupt::BOD_VLM),
            3 => Ok(Interrupt::CLKCTRL_CFD),
            4 => Ok(Interrupt::RTC_CNT),
            5 => Ok(Interrupt::RTC_PIT),
            6 => Ok(Interrupt::CCL_CCL),
            7 => Ok(Interrupt::USB0_BUSEVENT),
            8 => Ok(Interrupt::USB0_TRNCOMPL),
            9 => Ok(Interrupt::PORTA_PORT),
            10 => Ok(Interrupt::TCA0_LUNF_OVF),
            11 => Ok(Interrupt::TCA0_HUNF),
            12 => Ok(Interrupt::TCA0_CMP0_LCMP0),
            13 => Ok(Interrupt::TCA0_CMP1_LCMP1),
            14 => Ok(Interrupt::TCA0_CMP2_LCMP2),
            15 => Ok(Interrupt::TCB0_INT),
            16 => Ok(Interrupt::TWI0_TWIS),
            17 => Ok(Interrupt::TWI0_TWIM),
            18 => Ok(Interrupt::SPI0_INT),
            19 => Ok(Interrupt::USART0_RXC),
            20 => Ok(Interrupt::USART0_DRE),
            21 => Ok(Interrupt::USART0_TXC),
            22 => Ok(Interrupt::PORTD_PORT),
            23 => Ok(Interrupt::PORTC_PORT),
            24 => Ok(Interrupt::PORTF_PORT),
            25 => Ok(Interrupt::NVMCTRL_NVMREADY),
            26 => Ok(Interrupt::USART1_RXC),
            27 => Ok(Interrupt::USART1_DRE),
            28 => Ok(Interrupt::USART1_TXC),
            29 => Ok(Interrupt::TCB1_INT),
            30 => Ok(Interrupt::AC0_AC),
            31 => Ok(Interrupt::ADC0_ERROR),
            32 => Ok(Interrupt::ADC0_RESRDY),
            33 => Ok(Interrupt::ADC0_SAMPRDY),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
