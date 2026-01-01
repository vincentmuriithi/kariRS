#[doc = "Register `PITCTRLA` reader"]
pub struct R(crate::R<PITCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PITCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PITCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PITCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PITCTRLA` writer"]
pub struct W(crate::W<PITCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PITCTRLA_SPEC>;
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
impl From<crate::W<PITCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PITCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PITEN` reader - Enable"]
pub type PITEN_R = crate::BitReader<bool>;
#[doc = "Field `PITEN` writer - Enable"]
pub type PITEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, PITCTRLA_SPEC, bool, O>;
#[doc = "Field `PERIOD` reader - Period"]
pub type PERIOD_R = crate::FieldReader<u8, PERIOD_A>;
#[doc = "Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERIOD_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: RTC Clock Cycles 4"]
    CYC4 = 1,
    #[doc = "2: RTC Clock Cycles 8"]
    CYC8 = 2,
    #[doc = "3: RTC Clock Cycles 16"]
    CYC16 = 3,
    #[doc = "4: RTC Clock Cycles 32"]
    CYC32 = 4,
    #[doc = "5: RTC Clock Cycles 64"]
    CYC64 = 5,
    #[doc = "6: RTC Clock Cycles 128"]
    CYC128 = 6,
    #[doc = "7: RTC Clock Cycles 256"]
    CYC256 = 7,
    #[doc = "8: RTC Clock Cycles 512"]
    CYC512 = 8,
    #[doc = "9: RTC Clock Cycles 1024"]
    CYC1024 = 9,
    #[doc = "10: RTC Clock Cycles 2048"]
    CYC2048 = 10,
    #[doc = "11: RTC Clock Cycles 4096"]
    CYC4096 = 11,
    #[doc = "12: RTC Clock Cycles 8192"]
    CYC8192 = 12,
    #[doc = "13: RTC Clock Cycles 16384"]
    CYC16384 = 13,
    #[doc = "14: RTC Clock Cycles 32768"]
    CYC32768 = 14,
}
impl From<PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIOD_A) -> Self {
        variant as _
    }
}
impl PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERIOD_A> {
        match self.bits {
            0 => Some(PERIOD_A::OFF),
            1 => Some(PERIOD_A::CYC4),
            2 => Some(PERIOD_A::CYC8),
            3 => Some(PERIOD_A::CYC16),
            4 => Some(PERIOD_A::CYC32),
            5 => Some(PERIOD_A::CYC64),
            6 => Some(PERIOD_A::CYC128),
            7 => Some(PERIOD_A::CYC256),
            8 => Some(PERIOD_A::CYC512),
            9 => Some(PERIOD_A::CYC1024),
            10 => Some(PERIOD_A::CYC2048),
            11 => Some(PERIOD_A::CYC4096),
            12 => Some(PERIOD_A::CYC8192),
            13 => Some(PERIOD_A::CYC16384),
            14 => Some(PERIOD_A::CYC32768),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PERIOD_A::OFF
    }
    #[doc = "Checks if the value of the field is `CYC4`"]
    #[inline(always)]
    pub fn is_cyc4(&self) -> bool {
        *self == PERIOD_A::CYC4
    }
    #[doc = "Checks if the value of the field is `CYC8`"]
    #[inline(always)]
    pub fn is_cyc8(&self) -> bool {
        *self == PERIOD_A::CYC8
    }
    #[doc = "Checks if the value of the field is `CYC16`"]
    #[inline(always)]
    pub fn is_cyc16(&self) -> bool {
        *self == PERIOD_A::CYC16
    }
    #[doc = "Checks if the value of the field is `CYC32`"]
    #[inline(always)]
    pub fn is_cyc32(&self) -> bool {
        *self == PERIOD_A::CYC32
    }
    #[doc = "Checks if the value of the field is `CYC64`"]
    #[inline(always)]
    pub fn is_cyc64(&self) -> bool {
        *self == PERIOD_A::CYC64
    }
    #[doc = "Checks if the value of the field is `CYC128`"]
    #[inline(always)]
    pub fn is_cyc128(&self) -> bool {
        *self == PERIOD_A::CYC128
    }
    #[doc = "Checks if the value of the field is `CYC256`"]
    #[inline(always)]
    pub fn is_cyc256(&self) -> bool {
        *self == PERIOD_A::CYC256
    }
    #[doc = "Checks if the value of the field is `CYC512`"]
    #[inline(always)]
    pub fn is_cyc512(&self) -> bool {
        *self == PERIOD_A::CYC512
    }
    #[doc = "Checks if the value of the field is `CYC1024`"]
    #[inline(always)]
    pub fn is_cyc1024(&self) -> bool {
        *self == PERIOD_A::CYC1024
    }
    #[doc = "Checks if the value of the field is `CYC2048`"]
    #[inline(always)]
    pub fn is_cyc2048(&self) -> bool {
        *self == PERIOD_A::CYC2048
    }
    #[doc = "Checks if the value of the field is `CYC4096`"]
    #[inline(always)]
    pub fn is_cyc4096(&self) -> bool {
        *self == PERIOD_A::CYC4096
    }
    #[doc = "Checks if the value of the field is `CYC8192`"]
    #[inline(always)]
    pub fn is_cyc8192(&self) -> bool {
        *self == PERIOD_A::CYC8192
    }
    #[doc = "Checks if the value of the field is `CYC16384`"]
    #[inline(always)]
    pub fn is_cyc16384(&self) -> bool {
        *self == PERIOD_A::CYC16384
    }
    #[doc = "Checks if the value of the field is `CYC32768`"]
    #[inline(always)]
    pub fn is_cyc32768(&self) -> bool {
        *self == PERIOD_A::CYC32768
    }
}
#[doc = "Field `PERIOD` writer - Period"]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PITCTRLA_SPEC, u8, PERIOD_A, 4, O>;
impl<'a, const O: u8> PERIOD_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PERIOD_A::OFF)
    }
    #[doc = "RTC Clock Cycles 4"]
    #[inline(always)]
    pub fn cyc4(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC4)
    }
    #[doc = "RTC Clock Cycles 8"]
    #[inline(always)]
    pub fn cyc8(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC8)
    }
    #[doc = "RTC Clock Cycles 16"]
    #[inline(always)]
    pub fn cyc16(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC16)
    }
    #[doc = "RTC Clock Cycles 32"]
    #[inline(always)]
    pub fn cyc32(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC32)
    }
    #[doc = "RTC Clock Cycles 64"]
    #[inline(always)]
    pub fn cyc64(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC64)
    }
    #[doc = "RTC Clock Cycles 128"]
    #[inline(always)]
    pub fn cyc128(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC128)
    }
    #[doc = "RTC Clock Cycles 256"]
    #[inline(always)]
    pub fn cyc256(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC256)
    }
    #[doc = "RTC Clock Cycles 512"]
    #[inline(always)]
    pub fn cyc512(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC512)
    }
    #[doc = "RTC Clock Cycles 1024"]
    #[inline(always)]
    pub fn cyc1024(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC1024)
    }
    #[doc = "RTC Clock Cycles 2048"]
    #[inline(always)]
    pub fn cyc2048(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC2048)
    }
    #[doc = "RTC Clock Cycles 4096"]
    #[inline(always)]
    pub fn cyc4096(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC4096)
    }
    #[doc = "RTC Clock Cycles 8192"]
    #[inline(always)]
    pub fn cyc8192(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC8192)
    }
    #[doc = "RTC Clock Cycles 16384"]
    #[inline(always)]
    pub fn cyc16384(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC16384)
    }
    #[doc = "RTC Clock Cycles 32768"]
    #[inline(always)]
    pub fn cyc32768(self) -> &'a mut W {
        self.variant(PERIOD_A::CYC32768)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn piten(&self) -> PITEN_R {
        PITEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - Period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits >> 3) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn piten(&mut self) -> PITEN_W<0> {
        PITEN_W::new(self)
    }
    #[doc = "Bits 3:6 - Period"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<3> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIT Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitctrla](index.html) module"]
pub struct PITCTRLA_SPEC;
impl crate::RegisterSpec for PITCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pitctrla::R](R) reader structure"]
impl crate::Readable for PITCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pitctrla::W](W) writer structure"]
impl crate::Writable for PITCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PITCTRLA to value 0"]
impl crate::Resettable for PITCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
