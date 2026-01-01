#[doc = "Register `CLKPR` reader"]
pub struct R(crate::R<CLKPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKPR` writer"]
pub struct W(crate::W<CLKPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPR_SPEC>;
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
impl From<crate::W<CLKPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPS` reader - Clock Prescaler Select Bits"]
pub type CLKPS_R = crate::FieldReader<u8, CLKPS_A>;
#[doc = "Clock Prescaler Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKPS_A {
    #[doc = "0: 1"]
    _1 = 0,
    #[doc = "1: 2"]
    _2 = 1,
    #[doc = "2: 4"]
    _4 = 2,
    #[doc = "3: 8"]
    _8 = 3,
    #[doc = "4: 16"]
    _16 = 4,
    #[doc = "5: 32"]
    _32 = 5,
    #[doc = "6: 64"]
    _64 = 6,
    #[doc = "7: 128"]
    _128 = 7,
    #[doc = "8: 256"]
    _256 = 8,
}
impl From<CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKPS_A) -> Self {
        variant as _
    }
}
impl CLKPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKPS_A> {
        match self.bits {
            0 => Some(CLKPS_A::_1),
            1 => Some(CLKPS_A::_2),
            2 => Some(CLKPS_A::_4),
            3 => Some(CLKPS_A::_8),
            4 => Some(CLKPS_A::_16),
            5 => Some(CLKPS_A::_32),
            6 => Some(CLKPS_A::_64),
            7 => Some(CLKPS_A::_128),
            8 => Some(CLKPS_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKPS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CLKPS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == CLKPS_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CLKPS_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CLKPS_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == CLKPS_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == CLKPS_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == CLKPS_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == CLKPS_A::_256
    }
}
#[doc = "Field `CLKPS` writer - Clock Prescaler Select Bits"]
pub type CLKPS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLKPR_SPEC, u8, CLKPS_A, 4, O>;
impl<'a, const O: u8> CLKPS_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKPS_A::_1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CLKPS_A::_2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CLKPS_A::_4)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CLKPS_A::_8)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CLKPS_A::_16)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(CLKPS_A::_32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(CLKPS_A::_64)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(CLKPS_A::_128)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(CLKPS_A::_256)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Prescaler Select Bits"]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Prescaler Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<0> {
        CLKPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Prescale Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpr](index.html) module"]
pub struct CLKPR_SPEC;
impl crate::RegisterSpec for CLKPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkpr::R](R) reader structure"]
impl crate::Readable for CLKPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpr::W](W) writer structure"]
impl crate::Writable for CLKPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKPR to value 0"]
impl crate::Resettable for CLKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
