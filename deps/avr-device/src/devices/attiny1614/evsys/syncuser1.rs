#[doc = "Register `SYNCUSER1` reader"]
pub struct R(crate::R<SYNCUSER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCUSER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCUSER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCUSER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCUSER1` writer"]
pub struct W(crate::W<SYNCUSER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCUSER1_SPEC>;
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
impl From<crate::W<SYNCUSER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCUSER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCUSER1` reader - Synchronous User Ch 1 - USART0"]
pub type SYNCUSER1_R = crate::FieldReader<u8, SYNCUSER1_A>;
#[doc = "Synchronous User Ch 1 - USART0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCUSER1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "2: Synchronous Event Channel 1"]
    SYNCCH1 = 2,
}
impl From<SYNCUSER1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCUSER1_A) -> Self {
        variant as _
    }
}
impl SYNCUSER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCUSER1_A> {
        match self.bits {
            0 => Some(SYNCUSER1_A::OFF),
            1 => Some(SYNCUSER1_A::SYNCCH0),
            2 => Some(SYNCUSER1_A::SYNCCH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYNCUSER1_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == SYNCUSER1_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `SYNCCH1`"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == SYNCUSER1_A::SYNCCH1
    }
}
#[doc = "Field `SYNCUSER1` writer - Synchronous User Ch 1 - USART0"]
pub type SYNCUSER1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SYNCUSER1_SPEC, u8, SYNCUSER1_A, 8, O>;
impl<'a, const O: u8> SYNCUSER1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SYNCUSER1_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(SYNCUSER1_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut W {
        self.variant(SYNCUSER1_A::SYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronous User Ch 1 - USART0"]
    #[inline(always)]
    pub fn syncuser1(&self) -> SYNCUSER1_R {
        SYNCUSER1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous User Ch 1 - USART0"]
    #[inline(always)]
    #[must_use]
    pub fn syncuser1(&mut self) -> SYNCUSER1_W<0> {
        SYNCUSER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous User Ch 1 - USART0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncuser1](index.html) module"]
pub struct SYNCUSER1_SPEC;
impl crate::RegisterSpec for SYNCUSER1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syncuser1::R](R) reader structure"]
impl crate::Readable for SYNCUSER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncuser1::W](W) writer structure"]
impl crate::Writable for SYNCUSER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCUSER1 to value 0"]
impl crate::Resettable for SYNCUSER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
