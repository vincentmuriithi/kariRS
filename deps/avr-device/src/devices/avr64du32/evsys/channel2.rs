#[doc = "Register `CHANNEL2` reader"]
pub struct R(crate::R<CHANNEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL2` writer"]
pub struct W(crate::W<CHANNEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHANNEL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL` reader - Channel generator select"]
pub type CHANNEL_R = crate::FieldReader<u8, CHANNEL_A>;
#[doc = "Channel generator select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNEL_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: UPDI SYNCH Character"]
    UPDI_SYNCH = 1,
    #[doc = "6: Real Time Counter Overflow"]
    RTC_OVF = 6,
    #[doc = "7: Real Time Counter Compare"]
    RTC_CMP = 7,
    #[doc = "8: Real Time Counter Event Output 0"]
    RTC_EVGEN0 = 8,
    #[doc = "9: Real Time Counter Event Output 1"]
    RTC_EVGEN1 = 9,
    #[doc = "16: Configurable Custom Logic LUT0"]
    CCL_LUT0 = 16,
    #[doc = "17: Configurable Custom Logic LUT1"]
    CCL_LUT1 = 17,
    #[doc = "18: Configurable Custom Logic LUT2"]
    CCL_LUT2 = 18,
    #[doc = "19: Configurable Custom Logic LUT3"]
    CCL_LUT3 = 19,
    #[doc = "32: Analog Comparator 0 Out"]
    AC0_OUT = 32,
    #[doc = "36: ADC0 Result Ready"]
    ADC0_RESRDY = 36,
    #[doc = "37: ADC0 Sample Ready"]
    ADC0_SAMPRDY = 37,
    #[doc = "38: ADC0 Window Comparator"]
    ADC0_WCMP = 38,
    #[doc = "64: Port A Event 0"]
    PORTA_EVGEN0 = 64,
    #[doc = "65: Port A Event 1"]
    PORTA_EVGEN1 = 65,
    #[doc = "68: Port C Event 0"]
    PORTC_EVGEN0 = 68,
    #[doc = "69: Port C Event 1"]
    PORTC_EVGEN1 = 69,
    #[doc = "70: Port D Event 0"]
    PORTD_EVGEN0 = 70,
    #[doc = "71: Port D Event 1"]
    PORTD_EVGEN1 = 71,
    #[doc = "74: Port F Event 0"]
    PORTF_EVGEN0 = 74,
    #[doc = "75: Port F Event 1"]
    PORTF_EVGEN1 = 75,
    #[doc = "96: USART 0 XCK"]
    USART0_XCK = 96,
    #[doc = "97: USART 1 XCK"]
    USART1_XCK = 97,
    #[doc = "104: SPI 0 SCK"]
    SPI0_SCK = 104,
    #[doc = "128: Timer/Counter A0 Overflow / Low Byte Timer Underflow"]
    TCA0_OVF_LUNF = 128,
    #[doc = "129: Timer/Counter A0 High Byte Timer Underflow"]
    TCA0_HUNF = 129,
    #[doc = "132: Timer/Counter A0 Compare 0 / Low Byte Compare 0"]
    TCA0_CMP0_LCMP0 = 132,
    #[doc = "133: Timer/Counter A0 Compare 1 / Low Byte Compare 1"]
    TCA0_CMP1_LCMP1 = 133,
    #[doc = "134: Timer/Counter A0 Compare 2 / Low Byte Compare 2"]
    TCA0_CMP2_LCMP2 = 134,
    #[doc = "160: Timer/Counter B0 Capture"]
    TCB0_CAPT = 160,
    #[doc = "161: Timer/Counter B0 Overflow"]
    TCB0_OVF = 161,
    #[doc = "162: Timer/Counter B1 Capture"]
    TCB1_CAPT = 162,
    #[doc = "163: Timer/Counter B1 Overflow"]
    TCB1_OVF = 163,
    #[doc = "192: USB0 Setup Received"]
    USB0_SETUP = 192,
    #[doc = "193: USB0 SOF Received"]
    USB0_SOF = 193,
    #[doc = "194: USB0 CRC Error"]
    USB0_CRC = 194,
    #[doc = "195: USB0 Underflow / Overflow"]
    USB0_UNFOVF = 195,
    #[doc = "196: USB0 Data Byte Received"]
    USB0_RX = 196,
    #[doc = "197: USB0 Data Byte Transmitted"]
    USB0_TX = 197,
}
impl From<CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_A) -> Self {
        variant as _
    }
}
impl CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNEL_A> {
        match self.bits {
            0 => Some(CHANNEL_A::OFF),
            1 => Some(CHANNEL_A::UPDI_SYNCH),
            6 => Some(CHANNEL_A::RTC_OVF),
            7 => Some(CHANNEL_A::RTC_CMP),
            8 => Some(CHANNEL_A::RTC_EVGEN0),
            9 => Some(CHANNEL_A::RTC_EVGEN1),
            16 => Some(CHANNEL_A::CCL_LUT0),
            17 => Some(CHANNEL_A::CCL_LUT1),
            18 => Some(CHANNEL_A::CCL_LUT2),
            19 => Some(CHANNEL_A::CCL_LUT3),
            32 => Some(CHANNEL_A::AC0_OUT),
            36 => Some(CHANNEL_A::ADC0_RESRDY),
            37 => Some(CHANNEL_A::ADC0_SAMPRDY),
            38 => Some(CHANNEL_A::ADC0_WCMP),
            64 => Some(CHANNEL_A::PORTA_EVGEN0),
            65 => Some(CHANNEL_A::PORTA_EVGEN1),
            68 => Some(CHANNEL_A::PORTC_EVGEN0),
            69 => Some(CHANNEL_A::PORTC_EVGEN1),
            70 => Some(CHANNEL_A::PORTD_EVGEN0),
            71 => Some(CHANNEL_A::PORTD_EVGEN1),
            74 => Some(CHANNEL_A::PORTF_EVGEN0),
            75 => Some(CHANNEL_A::PORTF_EVGEN1),
            96 => Some(CHANNEL_A::USART0_XCK),
            97 => Some(CHANNEL_A::USART1_XCK),
            104 => Some(CHANNEL_A::SPI0_SCK),
            128 => Some(CHANNEL_A::TCA0_OVF_LUNF),
            129 => Some(CHANNEL_A::TCA0_HUNF),
            132 => Some(CHANNEL_A::TCA0_CMP0_LCMP0),
            133 => Some(CHANNEL_A::TCA0_CMP1_LCMP1),
            134 => Some(CHANNEL_A::TCA0_CMP2_LCMP2),
            160 => Some(CHANNEL_A::TCB0_CAPT),
            161 => Some(CHANNEL_A::TCB0_OVF),
            162 => Some(CHANNEL_A::TCB1_CAPT),
            163 => Some(CHANNEL_A::TCB1_OVF),
            192 => Some(CHANNEL_A::USB0_SETUP),
            193 => Some(CHANNEL_A::USB0_SOF),
            194 => Some(CHANNEL_A::USB0_CRC),
            195 => Some(CHANNEL_A::USB0_UNFOVF),
            196 => Some(CHANNEL_A::USB0_RX),
            197 => Some(CHANNEL_A::USB0_TX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CHANNEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `UPDI_SYNCH`"]
    #[inline(always)]
    pub fn is_updi_synch(&self) -> bool {
        *self == CHANNEL_A::UPDI_SYNCH
    }
    #[doc = "Checks if the value of the field is `RTC_OVF`"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == CHANNEL_A::RTC_OVF
    }
    #[doc = "Checks if the value of the field is `RTC_CMP`"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == CHANNEL_A::RTC_CMP
    }
    #[doc = "Checks if the value of the field is `RTC_EVGEN0`"]
    #[inline(always)]
    pub fn is_rtc_evgen0(&self) -> bool {
        *self == CHANNEL_A::RTC_EVGEN0
    }
    #[doc = "Checks if the value of the field is `RTC_EVGEN1`"]
    #[inline(always)]
    pub fn is_rtc_evgen1(&self) -> bool {
        *self == CHANNEL_A::RTC_EVGEN1
    }
    #[doc = "Checks if the value of the field is `CCL_LUT0`"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == CHANNEL_A::CCL_LUT0
    }
    #[doc = "Checks if the value of the field is `CCL_LUT1`"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == CHANNEL_A::CCL_LUT1
    }
    #[doc = "Checks if the value of the field is `CCL_LUT2`"]
    #[inline(always)]
    pub fn is_ccl_lut2(&self) -> bool {
        *self == CHANNEL_A::CCL_LUT2
    }
    #[doc = "Checks if the value of the field is `CCL_LUT3`"]
    #[inline(always)]
    pub fn is_ccl_lut3(&self) -> bool {
        *self == CHANNEL_A::CCL_LUT3
    }
    #[doc = "Checks if the value of the field is `AC0_OUT`"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == CHANNEL_A::AC0_OUT
    }
    #[doc = "Checks if the value of the field is `ADC0_RESRDY`"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == CHANNEL_A::ADC0_RESRDY
    }
    #[doc = "Checks if the value of the field is `ADC0_SAMPRDY`"]
    #[inline(always)]
    pub fn is_adc0_samprdy(&self) -> bool {
        *self == CHANNEL_A::ADC0_SAMPRDY
    }
    #[doc = "Checks if the value of the field is `ADC0_WCMP`"]
    #[inline(always)]
    pub fn is_adc0_wcmp(&self) -> bool {
        *self == CHANNEL_A::ADC0_WCMP
    }
    #[doc = "Checks if the value of the field is `PORTA_EVGEN0`"]
    #[inline(always)]
    pub fn is_porta_evgen0(&self) -> bool {
        *self == CHANNEL_A::PORTA_EVGEN0
    }
    #[doc = "Checks if the value of the field is `PORTA_EVGEN1`"]
    #[inline(always)]
    pub fn is_porta_evgen1(&self) -> bool {
        *self == CHANNEL_A::PORTA_EVGEN1
    }
    #[doc = "Checks if the value of the field is `PORTC_EVGEN0`"]
    #[inline(always)]
    pub fn is_portc_evgen0(&self) -> bool {
        *self == CHANNEL_A::PORTC_EVGEN0
    }
    #[doc = "Checks if the value of the field is `PORTC_EVGEN1`"]
    #[inline(always)]
    pub fn is_portc_evgen1(&self) -> bool {
        *self == CHANNEL_A::PORTC_EVGEN1
    }
    #[doc = "Checks if the value of the field is `PORTD_EVGEN0`"]
    #[inline(always)]
    pub fn is_portd_evgen0(&self) -> bool {
        *self == CHANNEL_A::PORTD_EVGEN0
    }
    #[doc = "Checks if the value of the field is `PORTD_EVGEN1`"]
    #[inline(always)]
    pub fn is_portd_evgen1(&self) -> bool {
        *self == CHANNEL_A::PORTD_EVGEN1
    }
    #[doc = "Checks if the value of the field is `PORTF_EVGEN0`"]
    #[inline(always)]
    pub fn is_portf_evgen0(&self) -> bool {
        *self == CHANNEL_A::PORTF_EVGEN0
    }
    #[doc = "Checks if the value of the field is `PORTF_EVGEN1`"]
    #[inline(always)]
    pub fn is_portf_evgen1(&self) -> bool {
        *self == CHANNEL_A::PORTF_EVGEN1
    }
    #[doc = "Checks if the value of the field is `USART0_XCK`"]
    #[inline(always)]
    pub fn is_usart0_xck(&self) -> bool {
        *self == CHANNEL_A::USART0_XCK
    }
    #[doc = "Checks if the value of the field is `USART1_XCK`"]
    #[inline(always)]
    pub fn is_usart1_xck(&self) -> bool {
        *self == CHANNEL_A::USART1_XCK
    }
    #[doc = "Checks if the value of the field is `SPI0_SCK`"]
    #[inline(always)]
    pub fn is_spi0_sck(&self) -> bool {
        *self == CHANNEL_A::SPI0_SCK
    }
    #[doc = "Checks if the value of the field is `TCA0_OVF_LUNF`"]
    #[inline(always)]
    pub fn is_tca0_ovf_lunf(&self) -> bool {
        *self == CHANNEL_A::TCA0_OVF_LUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_HUNF`"]
    #[inline(always)]
    pub fn is_tca0_hunf(&self) -> bool {
        *self == CHANNEL_A::TCA0_HUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP0_LCMP0`"]
    #[inline(always)]
    pub fn is_tca0_cmp0_lcmp0(&self) -> bool {
        *self == CHANNEL_A::TCA0_CMP0_LCMP0
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP1_LCMP1`"]
    #[inline(always)]
    pub fn is_tca0_cmp1_lcmp1(&self) -> bool {
        *self == CHANNEL_A::TCA0_CMP1_LCMP1
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP2_LCMP2`"]
    #[inline(always)]
    pub fn is_tca0_cmp2_lcmp2(&self) -> bool {
        *self == CHANNEL_A::TCA0_CMP2_LCMP2
    }
    #[doc = "Checks if the value of the field is `TCB0_CAPT`"]
    #[inline(always)]
    pub fn is_tcb0_capt(&self) -> bool {
        *self == CHANNEL_A::TCB0_CAPT
    }
    #[doc = "Checks if the value of the field is `TCB0_OVF`"]
    #[inline(always)]
    pub fn is_tcb0_ovf(&self) -> bool {
        *self == CHANNEL_A::TCB0_OVF
    }
    #[doc = "Checks if the value of the field is `TCB1_CAPT`"]
    #[inline(always)]
    pub fn is_tcb1_capt(&self) -> bool {
        *self == CHANNEL_A::TCB1_CAPT
    }
    #[doc = "Checks if the value of the field is `TCB1_OVF`"]
    #[inline(always)]
    pub fn is_tcb1_ovf(&self) -> bool {
        *self == CHANNEL_A::TCB1_OVF
    }
    #[doc = "Checks if the value of the field is `USB0_SETUP`"]
    #[inline(always)]
    pub fn is_usb0_setup(&self) -> bool {
        *self == CHANNEL_A::USB0_SETUP
    }
    #[doc = "Checks if the value of the field is `USB0_SOF`"]
    #[inline(always)]
    pub fn is_usb0_sof(&self) -> bool {
        *self == CHANNEL_A::USB0_SOF
    }
    #[doc = "Checks if the value of the field is `USB0_CRC`"]
    #[inline(always)]
    pub fn is_usb0_crc(&self) -> bool {
        *self == CHANNEL_A::USB0_CRC
    }
    #[doc = "Checks if the value of the field is `USB0_UNFOVF`"]
    #[inline(always)]
    pub fn is_usb0_unfovf(&self) -> bool {
        *self == CHANNEL_A::USB0_UNFOVF
    }
    #[doc = "Checks if the value of the field is `USB0_RX`"]
    #[inline(always)]
    pub fn is_usb0_rx(&self) -> bool {
        *self == CHANNEL_A::USB0_RX
    }
    #[doc = "Checks if the value of the field is `USB0_TX`"]
    #[inline(always)]
    pub fn is_usb0_tx(&self) -> bool {
        *self == CHANNEL_A::USB0_TX
    }
}
#[doc = "Field `CHANNEL` writer - Channel generator select"]
pub type CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CHANNEL2_SPEC, u8, CHANNEL_A, 8, O>;
impl<'a, const O: u8> CHANNEL_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CHANNEL_A::OFF)
    }
    #[doc = "UPDI SYNCH Character"]
    #[inline(always)]
    pub fn updi_synch(self) -> &'a mut W {
        self.variant(CHANNEL_A::UPDI_SYNCH)
    }
    #[doc = "Real Time Counter Overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut W {
        self.variant(CHANNEL_A::RTC_OVF)
    }
    #[doc = "Real Time Counter Compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut W {
        self.variant(CHANNEL_A::RTC_CMP)
    }
    #[doc = "Real Time Counter Event Output 0"]
    #[inline(always)]
    pub fn rtc_evgen0(self) -> &'a mut W {
        self.variant(CHANNEL_A::RTC_EVGEN0)
    }
    #[doc = "Real Time Counter Event Output 1"]
    #[inline(always)]
    pub fn rtc_evgen1(self) -> &'a mut W {
        self.variant(CHANNEL_A::RTC_EVGEN1)
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut W {
        self.variant(CHANNEL_A::CCL_LUT0)
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut W {
        self.variant(CHANNEL_A::CCL_LUT1)
    }
    #[doc = "Configurable Custom Logic LUT2"]
    #[inline(always)]
    pub fn ccl_lut2(self) -> &'a mut W {
        self.variant(CHANNEL_A::CCL_LUT2)
    }
    #[doc = "Configurable Custom Logic LUT3"]
    #[inline(always)]
    pub fn ccl_lut3(self) -> &'a mut W {
        self.variant(CHANNEL_A::CCL_LUT3)
    }
    #[doc = "Analog Comparator 0 Out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut W {
        self.variant(CHANNEL_A::AC0_OUT)
    }
    #[doc = "ADC0 Result Ready"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut W {
        self.variant(CHANNEL_A::ADC0_RESRDY)
    }
    #[doc = "ADC0 Sample Ready"]
    #[inline(always)]
    pub fn adc0_samprdy(self) -> &'a mut W {
        self.variant(CHANNEL_A::ADC0_SAMPRDY)
    }
    #[doc = "ADC0 Window Comparator"]
    #[inline(always)]
    pub fn adc0_wcmp(self) -> &'a mut W {
        self.variant(CHANNEL_A::ADC0_WCMP)
    }
    #[doc = "Port A Event 0"]
    #[inline(always)]
    pub fn porta_evgen0(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTA_EVGEN0)
    }
    #[doc = "Port A Event 1"]
    #[inline(always)]
    pub fn porta_evgen1(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTA_EVGEN1)
    }
    #[doc = "Port C Event 0"]
    #[inline(always)]
    pub fn portc_evgen0(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTC_EVGEN0)
    }
    #[doc = "Port C Event 1"]
    #[inline(always)]
    pub fn portc_evgen1(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTC_EVGEN1)
    }
    #[doc = "Port D Event 0"]
    #[inline(always)]
    pub fn portd_evgen0(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTD_EVGEN0)
    }
    #[doc = "Port D Event 1"]
    #[inline(always)]
    pub fn portd_evgen1(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTD_EVGEN1)
    }
    #[doc = "Port F Event 0"]
    #[inline(always)]
    pub fn portf_evgen0(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTF_EVGEN0)
    }
    #[doc = "Port F Event 1"]
    #[inline(always)]
    pub fn portf_evgen1(self) -> &'a mut W {
        self.variant(CHANNEL_A::PORTF_EVGEN1)
    }
    #[doc = "USART 0 XCK"]
    #[inline(always)]
    pub fn usart0_xck(self) -> &'a mut W {
        self.variant(CHANNEL_A::USART0_XCK)
    }
    #[doc = "USART 1 XCK"]
    #[inline(always)]
    pub fn usart1_xck(self) -> &'a mut W {
        self.variant(CHANNEL_A::USART1_XCK)
    }
    #[doc = "SPI 0 SCK"]
    #[inline(always)]
    pub fn spi0_sck(self) -> &'a mut W {
        self.variant(CHANNEL_A::SPI0_SCK)
    }
    #[doc = "Timer/Counter A0 Overflow / Low Byte Timer Underflow"]
    #[inline(always)]
    pub fn tca0_ovf_lunf(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCA0_OVF_LUNF)
    }
    #[doc = "Timer/Counter A0 High Byte Timer Underflow"]
    #[inline(always)]
    pub fn tca0_hunf(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCA0_HUNF)
    }
    #[doc = "Timer/Counter A0 Compare 0 / Low Byte Compare 0"]
    #[inline(always)]
    pub fn tca0_cmp0_lcmp0(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCA0_CMP0_LCMP0)
    }
    #[doc = "Timer/Counter A0 Compare 1 / Low Byte Compare 1"]
    #[inline(always)]
    pub fn tca0_cmp1_lcmp1(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCA0_CMP1_LCMP1)
    }
    #[doc = "Timer/Counter A0 Compare 2 / Low Byte Compare 2"]
    #[inline(always)]
    pub fn tca0_cmp2_lcmp2(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCA0_CMP2_LCMP2)
    }
    #[doc = "Timer/Counter B0 Capture"]
    #[inline(always)]
    pub fn tcb0_capt(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCB0_CAPT)
    }
    #[doc = "Timer/Counter B0 Overflow"]
    #[inline(always)]
    pub fn tcb0_ovf(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCB0_OVF)
    }
    #[doc = "Timer/Counter B1 Capture"]
    #[inline(always)]
    pub fn tcb1_capt(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCB1_CAPT)
    }
    #[doc = "Timer/Counter B1 Overflow"]
    #[inline(always)]
    pub fn tcb1_ovf(self) -> &'a mut W {
        self.variant(CHANNEL_A::TCB1_OVF)
    }
    #[doc = "USB0 Setup Received"]
    #[inline(always)]
    pub fn usb0_setup(self) -> &'a mut W {
        self.variant(CHANNEL_A::USB0_SETUP)
    }
    #[doc = "USB0 SOF Received"]
    #[inline(always)]
    pub fn usb0_sof(self) -> &'a mut W {
        self.variant(CHANNEL_A::USB0_SOF)
    }
    #[doc = "USB0 CRC Error"]
    #[inline(always)]
    pub fn usb0_crc(self) -> &'a mut W {
        self.variant(CHANNEL_A::USB0_CRC)
    }
    #[doc = "USB0 Underflow / Overflow"]
    #[inline(always)]
    pub fn usb0_unfovf(self) -> &'a mut W {
        self.variant(CHANNEL_A::USB0_UNFOVF)
    }
    #[doc = "USB0 Data Byte Received"]
    #[inline(always)]
    pub fn usb0_rx(self) -> &'a mut W {
        self.variant(CHANNEL_A::USB0_RX)
    }
    #[doc = "USB0 Data Byte Transmitted"]
    #[inline(always)]
    pub fn usb0_tx(self) -> &'a mut W {
        self.variant(CHANNEL_A::USB0_TX)
    }
}
impl R {
    #[doc = "Bits 0:7 - Channel generator select"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel generator select"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<0> {
        CHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multiplexer Channel 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel2](index.html) module"]
pub struct CHANNEL2_SPEC;
impl crate::RegisterSpec for CHANNEL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [channel2::R](R) reader structure"]
impl crate::Readable for CHANNEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel2::W](W) writer structure"]
impl crate::Writable for CHANNEL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL2 to value 0"]
impl crate::Resettable for CHANNEL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
