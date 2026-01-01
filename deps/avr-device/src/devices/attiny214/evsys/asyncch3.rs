#[doc = "Register `ASYNCCH3` reader"]
pub struct R(crate::R<ASYNCCH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCCH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCCH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCCH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCCH3` writer"]
pub struct W(crate::W<ASYNCCH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCCH3_SPEC>;
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
impl From<crate::W<ASYNCCH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCCH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCCH3` reader - Asynchronous Channel 3 Generator Selection"]
pub type ASYNCCH3_R = crate::FieldReader<u8, ASYNCCH3_A>;
#[doc = "Asynchronous Channel 3 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCCH3_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Configurable custom logic LUT0"]
    CCL_LUT0 = 1,
    #[doc = "2: Configurable custom logic LUT1"]
    CCL_LUT1 = 2,
    #[doc = "3: Analog Comparator 0 out"]
    AC0_OUT = 3,
    #[doc = "4: Timer/Counter type D compare B clear"]
    TCD0_CMPBCLR = 4,
    #[doc = "5: Timer/Counter type D compare A set"]
    TCD0_CMPASET = 5,
    #[doc = "6: Timer/Counter type D compare B set"]
    TCD0_CMPBSET = 6,
    #[doc = "7: Timer/Counter type D program event"]
    TCD0_PROGEV = 7,
    #[doc = "8: Real Time Counter overflow"]
    RTC_OVF = 8,
    #[doc = "9: Real Time Counter compare"]
    RTC_CMP = 9,
    #[doc = "10: Periodic Interrupt CLK_RTC div 8192"]
    PIT_DIV8192 = 10,
    #[doc = "11: Periodic Interrupt CLK_RTC div 4096"]
    PIT_DIV4096 = 11,
    #[doc = "12: Periodic Interrupt CLK_RTC div 2048"]
    PIT_DIV2048 = 12,
    #[doc = "13: Periodic Interrupt CLK_RTC div 1024"]
    PIT_DIV1024 = 13,
    #[doc = "14: Periodic Interrupt CLK_RTC div 512"]
    PIT_DIV512 = 14,
    #[doc = "15: Periodic Interrupt CLK_RTC div 256"]
    PIT_DIV256 = 15,
    #[doc = "16: Periodic Interrupt CLK_RTC div 128"]
    PIT_DIV128 = 16,
    #[doc = "17: Periodic Interrupt CLK_RTC div 64"]
    PIT_DIV64 = 17,
}
impl From<ASYNCCH3_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCCH3_A) -> Self {
        variant as _
    }
}
impl ASYNCCH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCCH3_A> {
        match self.bits {
            0 => Some(ASYNCCH3_A::OFF),
            1 => Some(ASYNCCH3_A::CCL_LUT0),
            2 => Some(ASYNCCH3_A::CCL_LUT1),
            3 => Some(ASYNCCH3_A::AC0_OUT),
            4 => Some(ASYNCCH3_A::TCD0_CMPBCLR),
            5 => Some(ASYNCCH3_A::TCD0_CMPASET),
            6 => Some(ASYNCCH3_A::TCD0_CMPBSET),
            7 => Some(ASYNCCH3_A::TCD0_PROGEV),
            8 => Some(ASYNCCH3_A::RTC_OVF),
            9 => Some(ASYNCCH3_A::RTC_CMP),
            10 => Some(ASYNCCH3_A::PIT_DIV8192),
            11 => Some(ASYNCCH3_A::PIT_DIV4096),
            12 => Some(ASYNCCH3_A::PIT_DIV2048),
            13 => Some(ASYNCCH3_A::PIT_DIV1024),
            14 => Some(ASYNCCH3_A::PIT_DIV512),
            15 => Some(ASYNCCH3_A::PIT_DIV256),
            16 => Some(ASYNCCH3_A::PIT_DIV128),
            17 => Some(ASYNCCH3_A::PIT_DIV64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCCH3_A::OFF
    }
    #[doc = "Checks if the value of the field is `CCL_LUT0`"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == ASYNCCH3_A::CCL_LUT0
    }
    #[doc = "Checks if the value of the field is `CCL_LUT1`"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == ASYNCCH3_A::CCL_LUT1
    }
    #[doc = "Checks if the value of the field is `AC0_OUT`"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == ASYNCCH3_A::AC0_OUT
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPBCLR`"]
    #[inline(always)]
    pub fn is_tcd0_cmpbclr(&self) -> bool {
        *self == ASYNCCH3_A::TCD0_CMPBCLR
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPASET`"]
    #[inline(always)]
    pub fn is_tcd0_cmpaset(&self) -> bool {
        *self == ASYNCCH3_A::TCD0_CMPASET
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPBSET`"]
    #[inline(always)]
    pub fn is_tcd0_cmpbset(&self) -> bool {
        *self == ASYNCCH3_A::TCD0_CMPBSET
    }
    #[doc = "Checks if the value of the field is `TCD0_PROGEV`"]
    #[inline(always)]
    pub fn is_tcd0_progev(&self) -> bool {
        *self == ASYNCCH3_A::TCD0_PROGEV
    }
    #[doc = "Checks if the value of the field is `RTC_OVF`"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == ASYNCCH3_A::RTC_OVF
    }
    #[doc = "Checks if the value of the field is `RTC_CMP`"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == ASYNCCH3_A::RTC_CMP
    }
    #[doc = "Checks if the value of the field is `PIT_DIV8192`"]
    #[inline(always)]
    pub fn is_pit_div8192(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV8192
    }
    #[doc = "Checks if the value of the field is `PIT_DIV4096`"]
    #[inline(always)]
    pub fn is_pit_div4096(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV4096
    }
    #[doc = "Checks if the value of the field is `PIT_DIV2048`"]
    #[inline(always)]
    pub fn is_pit_div2048(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV2048
    }
    #[doc = "Checks if the value of the field is `PIT_DIV1024`"]
    #[inline(always)]
    pub fn is_pit_div1024(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV1024
    }
    #[doc = "Checks if the value of the field is `PIT_DIV512`"]
    #[inline(always)]
    pub fn is_pit_div512(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV512
    }
    #[doc = "Checks if the value of the field is `PIT_DIV256`"]
    #[inline(always)]
    pub fn is_pit_div256(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV256
    }
    #[doc = "Checks if the value of the field is `PIT_DIV128`"]
    #[inline(always)]
    pub fn is_pit_div128(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV128
    }
    #[doc = "Checks if the value of the field is `PIT_DIV64`"]
    #[inline(always)]
    pub fn is_pit_div64(&self) -> bool {
        *self == ASYNCCH3_A::PIT_DIV64
    }
}
#[doc = "Field `ASYNCCH3` writer - Asynchronous Channel 3 Generator Selection"]
pub type ASYNCCH3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCCH3_SPEC, u8, ASYNCCH3_A, 8, O>;
impl<'a, const O: u8> ASYNCCH3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::OFF)
    }
    #[doc = "Configurable custom logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::CCL_LUT0)
    }
    #[doc = "Configurable custom logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::CCL_LUT1)
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::AC0_OUT)
    }
    #[doc = "Timer/Counter type D compare B clear"]
    #[inline(always)]
    pub fn tcd0_cmpbclr(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::TCD0_CMPBCLR)
    }
    #[doc = "Timer/Counter type D compare A set"]
    #[inline(always)]
    pub fn tcd0_cmpaset(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::TCD0_CMPASET)
    }
    #[doc = "Timer/Counter type D compare B set"]
    #[inline(always)]
    pub fn tcd0_cmpbset(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::TCD0_CMPBSET)
    }
    #[doc = "Timer/Counter type D program event"]
    #[inline(always)]
    pub fn tcd0_progev(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::TCD0_PROGEV)
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::RTC_OVF)
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::RTC_CMP)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 8192"]
    #[inline(always)]
    pub fn pit_div8192(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV8192)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 4096"]
    #[inline(always)]
    pub fn pit_div4096(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV4096)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 2048"]
    #[inline(always)]
    pub fn pit_div2048(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV2048)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 1024"]
    #[inline(always)]
    pub fn pit_div1024(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV1024)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 512"]
    #[inline(always)]
    pub fn pit_div512(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV512)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 256"]
    #[inline(always)]
    pub fn pit_div256(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV256)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 128"]
    #[inline(always)]
    pub fn pit_div128(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV128)
    }
    #[doc = "Periodic Interrupt CLK_RTC div 64"]
    #[inline(always)]
    pub fn pit_div64(self) -> &'a mut W {
        self.variant(ASYNCCH3_A::PIT_DIV64)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous Channel 3 Generator Selection"]
    #[inline(always)]
    pub fn asyncch3(&self) -> ASYNCCH3_R {
        ASYNCCH3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous Channel 3 Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn asyncch3(&mut self) -> ASYNCCH3_W<0> {
        ASYNCCH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous Channel 3 Generator Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncch3](index.html) module"]
pub struct ASYNCCH3_SPEC;
impl crate::RegisterSpec for ASYNCCH3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncch3::R](R) reader structure"]
impl crate::Readable for ASYNCCH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncch3::W](W) writer structure"]
impl crate::Writable for ASYNCCH3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCCH3 to value 0"]
impl crate::Resettable for ASYNCCH3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
