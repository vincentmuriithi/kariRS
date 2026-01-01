#[doc = "Register `ASYNCCH1` reader"]
pub struct R(crate::R<ASYNCCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCCH1` writer"]
pub struct W(crate::W<ASYNCCH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCCH1_SPEC>;
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
impl From<crate::W<ASYNCCH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCCH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCCH1` reader - Asynchronous Channel 1 Generator Selection"]
pub type ASYNCCH1_R = crate::FieldReader<u8, ASYNCCH1_A>;
#[doc = "Asynchronous Channel 1 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCCH1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Configurable custom logic LUT0"]
    CCL_LUT0 = 1,
    #[doc = "2: Configurable custom logic LUT1"]
    CCL_LUT1 = 2,
    #[doc = "3: Analog Comparator 0 out"]
    AC0_OUT = 3,
    #[doc = "4: Timer/Counter D0 compare B clear"]
    TCD0_CMPBCLR = 4,
    #[doc = "5: Timer/Counter D0 compare A set"]
    TCD0_CMPASET = 5,
    #[doc = "6: Timer/Counter D0 compare B set"]
    TCD0_CMPBSET = 6,
    #[doc = "7: Timer/Counter D0 program event"]
    TCD0_PROGEV = 7,
    #[doc = "8: Real Time Counter overflow"]
    RTC_OVF = 8,
    #[doc = "9: Real Time Counter compare"]
    RTC_CMP = 9,
    #[doc = "10: Asynchronous Event from Pin PB0"]
    PORTB_PIN0 = 10,
    #[doc = "11: Asynchronous Event from Pin PB1"]
    PORTB_PIN1 = 11,
    #[doc = "12: Asynchronous Event from Pin PB2"]
    PORTB_PIN2 = 12,
    #[doc = "13: Asynchronous Event from Pin PB3"]
    PORTB_PIN3 = 13,
    #[doc = "14: Asynchronous Event from Pin PB4"]
    PORTB_PIN4 = 14,
    #[doc = "15: Asynchronous Event from Pin PB5"]
    PORTB_PIN5 = 15,
    #[doc = "16: Asynchronous Event from Pin PB6"]
    PORTB_PIN6 = 16,
    #[doc = "17: Asynchronous Event from Pin PB7"]
    PORTB_PIN7 = 17,
    #[doc = "18: Analog Comparator 1 out"]
    AC1_OUT = 18,
    #[doc = "19: Analog Comparator 2 out"]
    AC2_OUT = 19,
}
impl From<ASYNCCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCCH1_A) -> Self {
        variant as _
    }
}
impl ASYNCCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCCH1_A> {
        match self.bits {
            0 => Some(ASYNCCH1_A::OFF),
            1 => Some(ASYNCCH1_A::CCL_LUT0),
            2 => Some(ASYNCCH1_A::CCL_LUT1),
            3 => Some(ASYNCCH1_A::AC0_OUT),
            4 => Some(ASYNCCH1_A::TCD0_CMPBCLR),
            5 => Some(ASYNCCH1_A::TCD0_CMPASET),
            6 => Some(ASYNCCH1_A::TCD0_CMPBSET),
            7 => Some(ASYNCCH1_A::TCD0_PROGEV),
            8 => Some(ASYNCCH1_A::RTC_OVF),
            9 => Some(ASYNCCH1_A::RTC_CMP),
            10 => Some(ASYNCCH1_A::PORTB_PIN0),
            11 => Some(ASYNCCH1_A::PORTB_PIN1),
            12 => Some(ASYNCCH1_A::PORTB_PIN2),
            13 => Some(ASYNCCH1_A::PORTB_PIN3),
            14 => Some(ASYNCCH1_A::PORTB_PIN4),
            15 => Some(ASYNCCH1_A::PORTB_PIN5),
            16 => Some(ASYNCCH1_A::PORTB_PIN6),
            17 => Some(ASYNCCH1_A::PORTB_PIN7),
            18 => Some(ASYNCCH1_A::AC1_OUT),
            19 => Some(ASYNCCH1_A::AC2_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCCH1_A::OFF
    }
    #[doc = "Checks if the value of the field is `CCL_LUT0`"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == ASYNCCH1_A::CCL_LUT0
    }
    #[doc = "Checks if the value of the field is `CCL_LUT1`"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == ASYNCCH1_A::CCL_LUT1
    }
    #[doc = "Checks if the value of the field is `AC0_OUT`"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == ASYNCCH1_A::AC0_OUT
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPBCLR`"]
    #[inline(always)]
    pub fn is_tcd0_cmpbclr(&self) -> bool {
        *self == ASYNCCH1_A::TCD0_CMPBCLR
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPASET`"]
    #[inline(always)]
    pub fn is_tcd0_cmpaset(&self) -> bool {
        *self == ASYNCCH1_A::TCD0_CMPASET
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPBSET`"]
    #[inline(always)]
    pub fn is_tcd0_cmpbset(&self) -> bool {
        *self == ASYNCCH1_A::TCD0_CMPBSET
    }
    #[doc = "Checks if the value of the field is `TCD0_PROGEV`"]
    #[inline(always)]
    pub fn is_tcd0_progev(&self) -> bool {
        *self == ASYNCCH1_A::TCD0_PROGEV
    }
    #[doc = "Checks if the value of the field is `RTC_OVF`"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == ASYNCCH1_A::RTC_OVF
    }
    #[doc = "Checks if the value of the field is `RTC_CMP`"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == ASYNCCH1_A::RTC_CMP
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN0`"]
    #[inline(always)]
    pub fn is_portb_pin0(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN0
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN1`"]
    #[inline(always)]
    pub fn is_portb_pin1(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN1
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN2`"]
    #[inline(always)]
    pub fn is_portb_pin2(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN2
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN3`"]
    #[inline(always)]
    pub fn is_portb_pin3(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN3
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN4`"]
    #[inline(always)]
    pub fn is_portb_pin4(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN4
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN5`"]
    #[inline(always)]
    pub fn is_portb_pin5(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN5
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN6`"]
    #[inline(always)]
    pub fn is_portb_pin6(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN6
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN7`"]
    #[inline(always)]
    pub fn is_portb_pin7(&self) -> bool {
        *self == ASYNCCH1_A::PORTB_PIN7
    }
    #[doc = "Checks if the value of the field is `AC1_OUT`"]
    #[inline(always)]
    pub fn is_ac1_out(&self) -> bool {
        *self == ASYNCCH1_A::AC1_OUT
    }
    #[doc = "Checks if the value of the field is `AC2_OUT`"]
    #[inline(always)]
    pub fn is_ac2_out(&self) -> bool {
        *self == ASYNCCH1_A::AC2_OUT
    }
}
#[doc = "Field `ASYNCCH1` writer - Asynchronous Channel 1 Generator Selection"]
pub type ASYNCCH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCCH1_SPEC, u8, ASYNCCH1_A, 8, O>;
impl<'a, const O: u8> ASYNCCH1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::OFF)
    }
    #[doc = "Configurable custom logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::CCL_LUT0)
    }
    #[doc = "Configurable custom logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::CCL_LUT1)
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::AC0_OUT)
    }
    #[doc = "Timer/Counter D0 compare B clear"]
    #[inline(always)]
    pub fn tcd0_cmpbclr(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::TCD0_CMPBCLR)
    }
    #[doc = "Timer/Counter D0 compare A set"]
    #[inline(always)]
    pub fn tcd0_cmpaset(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::TCD0_CMPASET)
    }
    #[doc = "Timer/Counter D0 compare B set"]
    #[inline(always)]
    pub fn tcd0_cmpbset(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::TCD0_CMPBSET)
    }
    #[doc = "Timer/Counter D0 program event"]
    #[inline(always)]
    pub fn tcd0_progev(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::TCD0_PROGEV)
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::RTC_OVF)
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::RTC_CMP)
    }
    #[doc = "Asynchronous Event from Pin PB0"]
    #[inline(always)]
    pub fn portb_pin0(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN0)
    }
    #[doc = "Asynchronous Event from Pin PB1"]
    #[inline(always)]
    pub fn portb_pin1(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN1)
    }
    #[doc = "Asynchronous Event from Pin PB2"]
    #[inline(always)]
    pub fn portb_pin2(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN2)
    }
    #[doc = "Asynchronous Event from Pin PB3"]
    #[inline(always)]
    pub fn portb_pin3(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN3)
    }
    #[doc = "Asynchronous Event from Pin PB4"]
    #[inline(always)]
    pub fn portb_pin4(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN4)
    }
    #[doc = "Asynchronous Event from Pin PB5"]
    #[inline(always)]
    pub fn portb_pin5(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN5)
    }
    #[doc = "Asynchronous Event from Pin PB6"]
    #[inline(always)]
    pub fn portb_pin6(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN6)
    }
    #[doc = "Asynchronous Event from Pin PB7"]
    #[inline(always)]
    pub fn portb_pin7(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::PORTB_PIN7)
    }
    #[doc = "Analog Comparator 1 out"]
    #[inline(always)]
    pub fn ac1_out(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::AC1_OUT)
    }
    #[doc = "Analog Comparator 2 out"]
    #[inline(always)]
    pub fn ac2_out(self) -> &'a mut W {
        self.variant(ASYNCCH1_A::AC2_OUT)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous Channel 1 Generator Selection"]
    #[inline(always)]
    pub fn asyncch1(&self) -> ASYNCCH1_R {
        ASYNCCH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous Channel 1 Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn asyncch1(&mut self) -> ASYNCCH1_W<0> {
        ASYNCCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous Channel 1 Generator Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncch1](index.html) module"]
pub struct ASYNCCH1_SPEC;
impl crate::RegisterSpec for ASYNCCH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncch1::R](R) reader structure"]
impl crate::Readable for ASYNCCH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncch1::W](W) writer structure"]
impl crate::Writable for ASYNCCH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCCH1 to value 0"]
impl crate::Resettable for ASYNCCH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
