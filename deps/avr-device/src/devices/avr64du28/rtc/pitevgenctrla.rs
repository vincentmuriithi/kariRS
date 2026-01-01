#[doc = "Register `PITEVGENCTRLA` reader"]
pub struct R(crate::R<PITEVGENCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PITEVGENCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PITEVGENCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PITEVGENCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PITEVGENCTRLA` writer"]
pub struct W(crate::W<PITEVGENCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PITEVGENCTRLA_SPEC>;
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
impl From<crate::W<PITEVGENCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PITEVGENCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVGEN0SEL` reader - Event Generation 0 Select"]
pub type EVGEN0SEL_R = crate::FieldReader<u8, EVGEN0SEL_A>;
#[doc = "Event Generation 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVGEN0SEL_A {
    #[doc = "0: No Event Generated"]
    OFF = 0,
    #[doc = "1: CLK_RTC divided by 4"]
    DIV4 = 1,
    #[doc = "2: CLK_RTC divided by 8"]
    DIV8 = 2,
    #[doc = "3: CLK_RTC divided by 16"]
    DIV16 = 3,
    #[doc = "4: CLK_RTC divided by 32"]
    DIV32 = 4,
    #[doc = "5: CLK_RTC divided by 64"]
    DIV64 = 5,
    #[doc = "6: CLK_RTC divided by 128"]
    DIV128 = 6,
    #[doc = "7: CLK_RTC divided by 256"]
    DIV256 = 7,
    #[doc = "8: CLK_RTC divided by 512"]
    DIV512 = 8,
    #[doc = "9: CLK_RTC divided by 1024"]
    DIV1024 = 9,
    #[doc = "10: CLK_RTC divided by 2048"]
    DIV2048 = 10,
    #[doc = "11: CLK_RTC divided by 4096"]
    DIV4096 = 11,
    #[doc = "12: CLK_RTC divided by 8192"]
    DIV8192 = 12,
    #[doc = "13: CLK_RTC divided by 16384"]
    DIV16384 = 13,
    #[doc = "14: CLK_RTC divided by 32768"]
    DIV32768 = 14,
}
impl From<EVGEN0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EVGEN0SEL_A) -> Self {
        variant as _
    }
}
impl EVGEN0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVGEN0SEL_A> {
        match self.bits {
            0 => Some(EVGEN0SEL_A::OFF),
            1 => Some(EVGEN0SEL_A::DIV4),
            2 => Some(EVGEN0SEL_A::DIV8),
            3 => Some(EVGEN0SEL_A::DIV16),
            4 => Some(EVGEN0SEL_A::DIV32),
            5 => Some(EVGEN0SEL_A::DIV64),
            6 => Some(EVGEN0SEL_A::DIV128),
            7 => Some(EVGEN0SEL_A::DIV256),
            8 => Some(EVGEN0SEL_A::DIV512),
            9 => Some(EVGEN0SEL_A::DIV1024),
            10 => Some(EVGEN0SEL_A::DIV2048),
            11 => Some(EVGEN0SEL_A::DIV4096),
            12 => Some(EVGEN0SEL_A::DIV8192),
            13 => Some(EVGEN0SEL_A::DIV16384),
            14 => Some(EVGEN0SEL_A::DIV32768),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVGEN0SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == EVGEN0SEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == EVGEN0SEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == EVGEN0SEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == EVGEN0SEL_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == EVGEN0SEL_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == EVGEN0SEL_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == EVGEN0SEL_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == EVGEN0SEL_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == EVGEN0SEL_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == EVGEN0SEL_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == EVGEN0SEL_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == EVGEN0SEL_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == EVGEN0SEL_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == EVGEN0SEL_A::DIV32768
    }
}
#[doc = "Field `EVGEN0SEL` writer - Event Generation 0 Select"]
pub type EVGEN0SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PITEVGENCTRLA_SPEC, u8, EVGEN0SEL_A, 4, O>;
impl<'a, const O: u8> EVGEN0SEL_W<'a, O> {
    #[doc = "No Event Generated"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::OFF)
    }
    #[doc = "CLK_RTC divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV4)
    }
    #[doc = "CLK_RTC divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV8)
    }
    #[doc = "CLK_RTC divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV16)
    }
    #[doc = "CLK_RTC divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV32)
    }
    #[doc = "CLK_RTC divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV64)
    }
    #[doc = "CLK_RTC divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV128)
    }
    #[doc = "CLK_RTC divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV256)
    }
    #[doc = "CLK_RTC divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV512)
    }
    #[doc = "CLK_RTC divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV1024)
    }
    #[doc = "CLK_RTC divided by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV2048)
    }
    #[doc = "CLK_RTC divided by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV4096)
    }
    #[doc = "CLK_RTC divided by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV8192)
    }
    #[doc = "CLK_RTC divided by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV16384)
    }
    #[doc = "CLK_RTC divided by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(EVGEN0SEL_A::DIV32768)
    }
}
#[doc = "Field `EVGEN1SEL` reader - Event Generation 1 Select"]
pub type EVGEN1SEL_R = crate::FieldReader<u8, EVGEN1SEL_A>;
#[doc = "Event Generation 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVGEN1SEL_A {
    #[doc = "0: No Event Generated"]
    OFF = 0,
    #[doc = "1: CLK_RTC divided by 4"]
    DIV4 = 1,
    #[doc = "2: CLK_RTC divided by 8"]
    DIV8 = 2,
    #[doc = "3: CLK_RTC divided by 16"]
    DIV16 = 3,
    #[doc = "4: CLK_RTC divided by 32"]
    DIV32 = 4,
    #[doc = "5: CLK_RTC divided by 64"]
    DIV64 = 5,
    #[doc = "6: CLK_RTC divided by 128"]
    DIV128 = 6,
    #[doc = "7: CLK_RTC divided by 256"]
    DIV256 = 7,
    #[doc = "8: CLK_RTC divided by 512"]
    DIV512 = 8,
    #[doc = "9: CLK_RTC divided by 1024"]
    DIV1024 = 9,
    #[doc = "10: CLK_RTC divided by 2048"]
    DIV2048 = 10,
    #[doc = "11: CLK_RTC divided by 4096"]
    DIV4096 = 11,
    #[doc = "12: CLK_RTC divided by 8192"]
    DIV8192 = 12,
    #[doc = "13: CLK_RTC divided by 16384"]
    DIV16384 = 13,
    #[doc = "14: CLK_RTC divided by 32768"]
    DIV32768 = 14,
}
impl From<EVGEN1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EVGEN1SEL_A) -> Self {
        variant as _
    }
}
impl EVGEN1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVGEN1SEL_A> {
        match self.bits {
            0 => Some(EVGEN1SEL_A::OFF),
            1 => Some(EVGEN1SEL_A::DIV4),
            2 => Some(EVGEN1SEL_A::DIV8),
            3 => Some(EVGEN1SEL_A::DIV16),
            4 => Some(EVGEN1SEL_A::DIV32),
            5 => Some(EVGEN1SEL_A::DIV64),
            6 => Some(EVGEN1SEL_A::DIV128),
            7 => Some(EVGEN1SEL_A::DIV256),
            8 => Some(EVGEN1SEL_A::DIV512),
            9 => Some(EVGEN1SEL_A::DIV1024),
            10 => Some(EVGEN1SEL_A::DIV2048),
            11 => Some(EVGEN1SEL_A::DIV4096),
            12 => Some(EVGEN1SEL_A::DIV8192),
            13 => Some(EVGEN1SEL_A::DIV16384),
            14 => Some(EVGEN1SEL_A::DIV32768),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVGEN1SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == EVGEN1SEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == EVGEN1SEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == EVGEN1SEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == EVGEN1SEL_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == EVGEN1SEL_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == EVGEN1SEL_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == EVGEN1SEL_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == EVGEN1SEL_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == EVGEN1SEL_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == EVGEN1SEL_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == EVGEN1SEL_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == EVGEN1SEL_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == EVGEN1SEL_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == EVGEN1SEL_A::DIV32768
    }
}
#[doc = "Field `EVGEN1SEL` writer - Event Generation 1 Select"]
pub type EVGEN1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PITEVGENCTRLA_SPEC, u8, EVGEN1SEL_A, 4, O>;
impl<'a, const O: u8> EVGEN1SEL_W<'a, O> {
    #[doc = "No Event Generated"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::OFF)
    }
    #[doc = "CLK_RTC divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV4)
    }
    #[doc = "CLK_RTC divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV8)
    }
    #[doc = "CLK_RTC divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV16)
    }
    #[doc = "CLK_RTC divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV32)
    }
    #[doc = "CLK_RTC divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV64)
    }
    #[doc = "CLK_RTC divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV128)
    }
    #[doc = "CLK_RTC divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV256)
    }
    #[doc = "CLK_RTC divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV512)
    }
    #[doc = "CLK_RTC divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV1024)
    }
    #[doc = "CLK_RTC divided by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV2048)
    }
    #[doc = "CLK_RTC divided by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV4096)
    }
    #[doc = "CLK_RTC divided by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV8192)
    }
    #[doc = "CLK_RTC divided by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV16384)
    }
    #[doc = "CLK_RTC divided by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(EVGEN1SEL_A::DIV32768)
    }
}
impl R {
    #[doc = "Bits 0:3 - Event Generation 0 Select"]
    #[inline(always)]
    pub fn evgen0sel(&self) -> EVGEN0SEL_R {
        EVGEN0SEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Event Generation 1 Select"]
    #[inline(always)]
    pub fn evgen1sel(&self) -> EVGEN1SEL_R {
        EVGEN1SEL_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event Generation 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn evgen0sel(&mut self) -> EVGEN0SEL_W<0> {
        EVGEN0SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Event Generation 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn evgen1sel(&mut self) -> EVGEN1SEL_W<4> {
        EVGEN1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIT Event Generation Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitevgenctrla](index.html) module"]
pub struct PITEVGENCTRLA_SPEC;
impl crate::RegisterSpec for PITEVGENCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pitevgenctrla::R](R) reader structure"]
impl crate::Readable for PITEVGENCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pitevgenctrla::W](W) writer structure"]
impl crate::Writable for PITEVGENCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PITEVGENCTRLA to value 0"]
impl crate::Resettable for PITEVGENCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
