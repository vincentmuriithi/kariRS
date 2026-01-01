#[doc = "Register `CHANNEL0` reader"]
pub struct R(crate::R<CHANNEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL0` writer"]
pub struct W(crate::W<CHANNEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL0_SPEC>;
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
impl From<crate::W<CHANNEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GENERATOR` reader - Generator selector"]
pub type GENERATOR_R = crate::FieldReader<u8, GENERATOR_A>;
#[doc = "Generator selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GENERATOR_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Unified Program and Debug Interface"]
    UPDI = 1,
    #[doc = "6: Real Time Counter overflow"]
    RTC_OVF = 6,
    #[doc = "7: Real Time Counter compare"]
    RTC_CMP = 7,
    #[doc = "8: Periodic Interrupt Timer output 0"]
    RTC_PIT0 = 8,
    #[doc = "9: Periodic Interrupt Timer output 1"]
    RTC_PIT1 = 9,
    #[doc = "10: Periodic Interrupt Timer output 2"]
    RTC_PIT2 = 10,
    #[doc = "11: Periodic Interrupt Timer output 3"]
    RTC_PIT3 = 11,
    #[doc = "16: Configurable Custom Logic LUT0"]
    CCL_LUT0 = 16,
    #[doc = "17: Configurable Custom Logic LUT1"]
    CCL_LUT1 = 17,
    #[doc = "18: Configurable Custom Logic LUT2"]
    CCL_LUT2 = 18,
    #[doc = "19: Configurable Custom Logic LUT3"]
    CCL_LUT3 = 19,
    #[doc = "32: Analog Comparator 0 out"]
    AC0_OUT = 32,
    #[doc = "36: ADC 0 Result Ready Event"]
    ADC0_RESRDY = 36,
    #[doc = "64: Port 0 Pin 0"]
    PORT0_PIN0 = 64,
    #[doc = "65: Port 0 Pin 1"]
    PORT0_PIN1 = 65,
    #[doc = "66: Port 0 Pin 2"]
    PORT0_PIN2 = 66,
    #[doc = "67: Port 0 Pin 3"]
    PORT0_PIN3 = 67,
    #[doc = "68: Port 0 Pin 4"]
    PORT0_PIN4 = 68,
    #[doc = "69: Port 0 Pin 5"]
    PORT0_PIN5 = 69,
    #[doc = "70: Port 0 Pin 6"]
    PORT0_PIN6 = 70,
    #[doc = "71: Port 0 Pin 7"]
    PORT0_PIN7 = 71,
    #[doc = "72: Port 1 Pin 0"]
    PORT1_PIN0 = 72,
    #[doc = "73: Port 1 Pin 1"]
    PORT1_PIN1 = 73,
    #[doc = "74: Port 1 Pin 2"]
    PORT1_PIN2 = 74,
    #[doc = "75: Port 1 Pin 3"]
    PORT1_PIN3 = 75,
    #[doc = "76: Port 1 Pin 4"]
    PORT1_PIN4 = 76,
    #[doc = "77: Port 1 Pin 5"]
    PORT1_PIN5 = 77,
    #[doc = "78: Port 1 Pin 6"]
    PORT1_PIN6 = 78,
    #[doc = "79: Port 1 Pin 7"]
    PORT1_PIN7 = 79,
    #[doc = "96: USART 0 Xclock"]
    USART0_XCK = 96,
    #[doc = "97: USART 1 Xclock"]
    USART1_XCK = 97,
    #[doc = "98: USART 2 Xclock"]
    USART2_XCK = 98,
    #[doc = "99: USART 3 Xclock"]
    USART3_XCK = 99,
    #[doc = "104: SPI 0 Sclock"]
    SPI0_SCK = 104,
    #[doc = "128: Timer/Counter A0 overflow / low byte underflow"]
    TCA0_OVF_LUNF = 128,
    #[doc = "129: Timer/Counter A0 high byte underflow (split mode)"]
    TCA0_HUNF = 129,
    #[doc = "132: Timer/Counter A0 compare 0"]
    TCA0_CMP0 = 132,
    #[doc = "133: Timer/Counter A0 compare 1"]
    TCA0_CMP1 = 133,
    #[doc = "134: Timer/Counter A0 compare 2"]
    TCA0_CMP2 = 134,
    #[doc = "160: Timer/Counter B0 capture"]
    TCB0_CAPT = 160,
    #[doc = "162: Timer/Counter B1 capture"]
    TCB1_CAPT = 162,
    #[doc = "164: Timer/Counter B2 capture"]
    TCB2_CAPT = 164,
    #[doc = "166: Timer/Counter B3 capture"]
    TCB3_CAPT = 166,
}
impl From<GENERATOR_A> for u8 {
    #[inline(always)]
    fn from(variant: GENERATOR_A) -> Self {
        variant as _
    }
}
impl GENERATOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENERATOR_A> {
        match self.bits {
            0 => Some(GENERATOR_A::OFF),
            1 => Some(GENERATOR_A::UPDI),
            6 => Some(GENERATOR_A::RTC_OVF),
            7 => Some(GENERATOR_A::RTC_CMP),
            8 => Some(GENERATOR_A::RTC_PIT0),
            9 => Some(GENERATOR_A::RTC_PIT1),
            10 => Some(GENERATOR_A::RTC_PIT2),
            11 => Some(GENERATOR_A::RTC_PIT3),
            16 => Some(GENERATOR_A::CCL_LUT0),
            17 => Some(GENERATOR_A::CCL_LUT1),
            18 => Some(GENERATOR_A::CCL_LUT2),
            19 => Some(GENERATOR_A::CCL_LUT3),
            32 => Some(GENERATOR_A::AC0_OUT),
            36 => Some(GENERATOR_A::ADC0_RESRDY),
            64 => Some(GENERATOR_A::PORT0_PIN0),
            65 => Some(GENERATOR_A::PORT0_PIN1),
            66 => Some(GENERATOR_A::PORT0_PIN2),
            67 => Some(GENERATOR_A::PORT0_PIN3),
            68 => Some(GENERATOR_A::PORT0_PIN4),
            69 => Some(GENERATOR_A::PORT0_PIN5),
            70 => Some(GENERATOR_A::PORT0_PIN6),
            71 => Some(GENERATOR_A::PORT0_PIN7),
            72 => Some(GENERATOR_A::PORT1_PIN0),
            73 => Some(GENERATOR_A::PORT1_PIN1),
            74 => Some(GENERATOR_A::PORT1_PIN2),
            75 => Some(GENERATOR_A::PORT1_PIN3),
            76 => Some(GENERATOR_A::PORT1_PIN4),
            77 => Some(GENERATOR_A::PORT1_PIN5),
            78 => Some(GENERATOR_A::PORT1_PIN6),
            79 => Some(GENERATOR_A::PORT1_PIN7),
            96 => Some(GENERATOR_A::USART0_XCK),
            97 => Some(GENERATOR_A::USART1_XCK),
            98 => Some(GENERATOR_A::USART2_XCK),
            99 => Some(GENERATOR_A::USART3_XCK),
            104 => Some(GENERATOR_A::SPI0_SCK),
            128 => Some(GENERATOR_A::TCA0_OVF_LUNF),
            129 => Some(GENERATOR_A::TCA0_HUNF),
            132 => Some(GENERATOR_A::TCA0_CMP0),
            133 => Some(GENERATOR_A::TCA0_CMP1),
            134 => Some(GENERATOR_A::TCA0_CMP2),
            160 => Some(GENERATOR_A::TCB0_CAPT),
            162 => Some(GENERATOR_A::TCB1_CAPT),
            164 => Some(GENERATOR_A::TCB2_CAPT),
            166 => Some(GENERATOR_A::TCB3_CAPT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == GENERATOR_A::OFF
    }
    #[doc = "Checks if the value of the field is `UPDI`"]
    #[inline(always)]
    pub fn is_updi(&self) -> bool {
        *self == GENERATOR_A::UPDI
    }
    #[doc = "Checks if the value of the field is `RTC_OVF`"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == GENERATOR_A::RTC_OVF
    }
    #[doc = "Checks if the value of the field is `RTC_CMP`"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == GENERATOR_A::RTC_CMP
    }
    #[doc = "Checks if the value of the field is `RTC_PIT0`"]
    #[inline(always)]
    pub fn is_rtc_pit0(&self) -> bool {
        *self == GENERATOR_A::RTC_PIT0
    }
    #[doc = "Checks if the value of the field is `RTC_PIT1`"]
    #[inline(always)]
    pub fn is_rtc_pit1(&self) -> bool {
        *self == GENERATOR_A::RTC_PIT1
    }
    #[doc = "Checks if the value of the field is `RTC_PIT2`"]
    #[inline(always)]
    pub fn is_rtc_pit2(&self) -> bool {
        *self == GENERATOR_A::RTC_PIT2
    }
    #[doc = "Checks if the value of the field is `RTC_PIT3`"]
    #[inline(always)]
    pub fn is_rtc_pit3(&self) -> bool {
        *self == GENERATOR_A::RTC_PIT3
    }
    #[doc = "Checks if the value of the field is `CCL_LUT0`"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == GENERATOR_A::CCL_LUT0
    }
    #[doc = "Checks if the value of the field is `CCL_LUT1`"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == GENERATOR_A::CCL_LUT1
    }
    #[doc = "Checks if the value of the field is `CCL_LUT2`"]
    #[inline(always)]
    pub fn is_ccl_lut2(&self) -> bool {
        *self == GENERATOR_A::CCL_LUT2
    }
    #[doc = "Checks if the value of the field is `CCL_LUT3`"]
    #[inline(always)]
    pub fn is_ccl_lut3(&self) -> bool {
        *self == GENERATOR_A::CCL_LUT3
    }
    #[doc = "Checks if the value of the field is `AC0_OUT`"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == GENERATOR_A::AC0_OUT
    }
    #[doc = "Checks if the value of the field is `ADC0_RESRDY`"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == GENERATOR_A::ADC0_RESRDY
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN0`"]
    #[inline(always)]
    pub fn is_port0_pin0(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN0
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN1`"]
    #[inline(always)]
    pub fn is_port0_pin1(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN1
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN2`"]
    #[inline(always)]
    pub fn is_port0_pin2(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN2
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN3`"]
    #[inline(always)]
    pub fn is_port0_pin3(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN3
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN4`"]
    #[inline(always)]
    pub fn is_port0_pin4(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN4
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN5`"]
    #[inline(always)]
    pub fn is_port0_pin5(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN5
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN6`"]
    #[inline(always)]
    pub fn is_port0_pin6(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN6
    }
    #[doc = "Checks if the value of the field is `PORT0_PIN7`"]
    #[inline(always)]
    pub fn is_port0_pin7(&self) -> bool {
        *self == GENERATOR_A::PORT0_PIN7
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN0`"]
    #[inline(always)]
    pub fn is_port1_pin0(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN0
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN1`"]
    #[inline(always)]
    pub fn is_port1_pin1(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN1
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN2`"]
    #[inline(always)]
    pub fn is_port1_pin2(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN2
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN3`"]
    #[inline(always)]
    pub fn is_port1_pin3(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN3
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN4`"]
    #[inline(always)]
    pub fn is_port1_pin4(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN4
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN5`"]
    #[inline(always)]
    pub fn is_port1_pin5(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN5
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN6`"]
    #[inline(always)]
    pub fn is_port1_pin6(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN6
    }
    #[doc = "Checks if the value of the field is `PORT1_PIN7`"]
    #[inline(always)]
    pub fn is_port1_pin7(&self) -> bool {
        *self == GENERATOR_A::PORT1_PIN7
    }
    #[doc = "Checks if the value of the field is `USART0_XCK`"]
    #[inline(always)]
    pub fn is_usart0_xck(&self) -> bool {
        *self == GENERATOR_A::USART0_XCK
    }
    #[doc = "Checks if the value of the field is `USART1_XCK`"]
    #[inline(always)]
    pub fn is_usart1_xck(&self) -> bool {
        *self == GENERATOR_A::USART1_XCK
    }
    #[doc = "Checks if the value of the field is `USART2_XCK`"]
    #[inline(always)]
    pub fn is_usart2_xck(&self) -> bool {
        *self == GENERATOR_A::USART2_XCK
    }
    #[doc = "Checks if the value of the field is `USART3_XCK`"]
    #[inline(always)]
    pub fn is_usart3_xck(&self) -> bool {
        *self == GENERATOR_A::USART3_XCK
    }
    #[doc = "Checks if the value of the field is `SPI0_SCK`"]
    #[inline(always)]
    pub fn is_spi0_sck(&self) -> bool {
        *self == GENERATOR_A::SPI0_SCK
    }
    #[doc = "Checks if the value of the field is `TCA0_OVF_LUNF`"]
    #[inline(always)]
    pub fn is_tca0_ovf_lunf(&self) -> bool {
        *self == GENERATOR_A::TCA0_OVF_LUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_HUNF`"]
    #[inline(always)]
    pub fn is_tca0_hunf(&self) -> bool {
        *self == GENERATOR_A::TCA0_HUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP0`"]
    #[inline(always)]
    pub fn is_tca0_cmp0(&self) -> bool {
        *self == GENERATOR_A::TCA0_CMP0
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP1`"]
    #[inline(always)]
    pub fn is_tca0_cmp1(&self) -> bool {
        *self == GENERATOR_A::TCA0_CMP1
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP2`"]
    #[inline(always)]
    pub fn is_tca0_cmp2(&self) -> bool {
        *self == GENERATOR_A::TCA0_CMP2
    }
    #[doc = "Checks if the value of the field is `TCB0_CAPT`"]
    #[inline(always)]
    pub fn is_tcb0_capt(&self) -> bool {
        *self == GENERATOR_A::TCB0_CAPT
    }
    #[doc = "Checks if the value of the field is `TCB1_CAPT`"]
    #[inline(always)]
    pub fn is_tcb1_capt(&self) -> bool {
        *self == GENERATOR_A::TCB1_CAPT
    }
    #[doc = "Checks if the value of the field is `TCB2_CAPT`"]
    #[inline(always)]
    pub fn is_tcb2_capt(&self) -> bool {
        *self == GENERATOR_A::TCB2_CAPT
    }
    #[doc = "Checks if the value of the field is `TCB3_CAPT`"]
    #[inline(always)]
    pub fn is_tcb3_capt(&self) -> bool {
        *self == GENERATOR_A::TCB3_CAPT
    }
}
#[doc = "Field `GENERATOR` writer - Generator selector"]
pub type GENERATOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CHANNEL0_SPEC, u8, GENERATOR_A, 8, O>;
impl<'a, const O: u8> GENERATOR_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(GENERATOR_A::OFF)
    }
    #[doc = "Unified Program and Debug Interface"]
    #[inline(always)]
    pub fn updi(self) -> &'a mut W {
        self.variant(GENERATOR_A::UPDI)
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut W {
        self.variant(GENERATOR_A::RTC_OVF)
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut W {
        self.variant(GENERATOR_A::RTC_CMP)
    }
    #[doc = "Periodic Interrupt Timer output 0"]
    #[inline(always)]
    pub fn rtc_pit0(self) -> &'a mut W {
        self.variant(GENERATOR_A::RTC_PIT0)
    }
    #[doc = "Periodic Interrupt Timer output 1"]
    #[inline(always)]
    pub fn rtc_pit1(self) -> &'a mut W {
        self.variant(GENERATOR_A::RTC_PIT1)
    }
    #[doc = "Periodic Interrupt Timer output 2"]
    #[inline(always)]
    pub fn rtc_pit2(self) -> &'a mut W {
        self.variant(GENERATOR_A::RTC_PIT2)
    }
    #[doc = "Periodic Interrupt Timer output 3"]
    #[inline(always)]
    pub fn rtc_pit3(self) -> &'a mut W {
        self.variant(GENERATOR_A::RTC_PIT3)
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut W {
        self.variant(GENERATOR_A::CCL_LUT0)
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut W {
        self.variant(GENERATOR_A::CCL_LUT1)
    }
    #[doc = "Configurable Custom Logic LUT2"]
    #[inline(always)]
    pub fn ccl_lut2(self) -> &'a mut W {
        self.variant(GENERATOR_A::CCL_LUT2)
    }
    #[doc = "Configurable Custom Logic LUT3"]
    #[inline(always)]
    pub fn ccl_lut3(self) -> &'a mut W {
        self.variant(GENERATOR_A::CCL_LUT3)
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut W {
        self.variant(GENERATOR_A::AC0_OUT)
    }
    #[doc = "ADC 0 Result Ready Event"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut W {
        self.variant(GENERATOR_A::ADC0_RESRDY)
    }
    #[doc = "Port 0 Pin 0"]
    #[inline(always)]
    pub fn port0_pin0(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN0)
    }
    #[doc = "Port 0 Pin 1"]
    #[inline(always)]
    pub fn port0_pin1(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN1)
    }
    #[doc = "Port 0 Pin 2"]
    #[inline(always)]
    pub fn port0_pin2(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN2)
    }
    #[doc = "Port 0 Pin 3"]
    #[inline(always)]
    pub fn port0_pin3(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN3)
    }
    #[doc = "Port 0 Pin 4"]
    #[inline(always)]
    pub fn port0_pin4(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN4)
    }
    #[doc = "Port 0 Pin 5"]
    #[inline(always)]
    pub fn port0_pin5(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN5)
    }
    #[doc = "Port 0 Pin 6"]
    #[inline(always)]
    pub fn port0_pin6(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN6)
    }
    #[doc = "Port 0 Pin 7"]
    #[inline(always)]
    pub fn port0_pin7(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT0_PIN7)
    }
    #[doc = "Port 1 Pin 0"]
    #[inline(always)]
    pub fn port1_pin0(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN0)
    }
    #[doc = "Port 1 Pin 1"]
    #[inline(always)]
    pub fn port1_pin1(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN1)
    }
    #[doc = "Port 1 Pin 2"]
    #[inline(always)]
    pub fn port1_pin2(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN2)
    }
    #[doc = "Port 1 Pin 3"]
    #[inline(always)]
    pub fn port1_pin3(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN3)
    }
    #[doc = "Port 1 Pin 4"]
    #[inline(always)]
    pub fn port1_pin4(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN4)
    }
    #[doc = "Port 1 Pin 5"]
    #[inline(always)]
    pub fn port1_pin5(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN5)
    }
    #[doc = "Port 1 Pin 6"]
    #[inline(always)]
    pub fn port1_pin6(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN6)
    }
    #[doc = "Port 1 Pin 7"]
    #[inline(always)]
    pub fn port1_pin7(self) -> &'a mut W {
        self.variant(GENERATOR_A::PORT1_PIN7)
    }
    #[doc = "USART 0 Xclock"]
    #[inline(always)]
    pub fn usart0_xck(self) -> &'a mut W {
        self.variant(GENERATOR_A::USART0_XCK)
    }
    #[doc = "USART 1 Xclock"]
    #[inline(always)]
    pub fn usart1_xck(self) -> &'a mut W {
        self.variant(GENERATOR_A::USART1_XCK)
    }
    #[doc = "USART 2 Xclock"]
    #[inline(always)]
    pub fn usart2_xck(self) -> &'a mut W {
        self.variant(GENERATOR_A::USART2_XCK)
    }
    #[doc = "USART 3 Xclock"]
    #[inline(always)]
    pub fn usart3_xck(self) -> &'a mut W {
        self.variant(GENERATOR_A::USART3_XCK)
    }
    #[doc = "SPI 0 Sclock"]
    #[inline(always)]
    pub fn spi0_sck(self) -> &'a mut W {
        self.variant(GENERATOR_A::SPI0_SCK)
    }
    #[doc = "Timer/Counter A0 overflow / low byte underflow"]
    #[inline(always)]
    pub fn tca0_ovf_lunf(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCA0_OVF_LUNF)
    }
    #[doc = "Timer/Counter A0 high byte underflow (split mode)"]
    #[inline(always)]
    pub fn tca0_hunf(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCA0_HUNF)
    }
    #[doc = "Timer/Counter A0 compare 0"]
    #[inline(always)]
    pub fn tca0_cmp0(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCA0_CMP0)
    }
    #[doc = "Timer/Counter A0 compare 1"]
    #[inline(always)]
    pub fn tca0_cmp1(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCA0_CMP1)
    }
    #[doc = "Timer/Counter A0 compare 2"]
    #[inline(always)]
    pub fn tca0_cmp2(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCA0_CMP2)
    }
    #[doc = "Timer/Counter B0 capture"]
    #[inline(always)]
    pub fn tcb0_capt(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCB0_CAPT)
    }
    #[doc = "Timer/Counter B1 capture"]
    #[inline(always)]
    pub fn tcb1_capt(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCB1_CAPT)
    }
    #[doc = "Timer/Counter B2 capture"]
    #[inline(always)]
    pub fn tcb2_capt(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCB2_CAPT)
    }
    #[doc = "Timer/Counter B3 capture"]
    #[inline(always)]
    pub fn tcb3_capt(self) -> &'a mut W {
        self.variant(GENERATOR_A::TCB3_CAPT)
    }
}
impl R {
    #[doc = "Bits 0:7 - Generator selector"]
    #[inline(always)]
    pub fn generator(&self) -> GENERATOR_R {
        GENERATOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Generator selector"]
    #[inline(always)]
    #[must_use]
    pub fn generator(&mut self) -> GENERATOR_W<0> {
        GENERATOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multiplexer Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel0](index.html) module"]
pub struct CHANNEL0_SPEC;
impl crate::RegisterSpec for CHANNEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [channel0::R](R) reader structure"]
impl crate::Readable for CHANNEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel0::W](W) writer structure"]
impl crate::Writable for CHANNEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL0 to value 0"]
impl crate::Resettable for CHANNEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
