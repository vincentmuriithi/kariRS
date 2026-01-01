#[doc = "Register `SYNCUSER0` reader"]
pub struct R(crate::R<SYNCUSER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCUSER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCUSER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCUSER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCUSER0` writer"]
pub struct W(crate::W<SYNCUSER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCUSER0_SPEC>;
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
impl From<crate::W<SYNCUSER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCUSER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCUSER0` reader - Synchronous User Ch 0 Input Selection - TCA0"]
pub type SYNCUSER0_R = crate::FieldReader<u8, SYNCUSER0_A>;
#[doc = "Synchronous User Ch 0 Input Selection - TCA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCUSER0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "2: Synchronous Event Channel 1"]
    SYNCCH1 = 2,
}
impl From<SYNCUSER0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCUSER0_A) -> Self {
        variant as _
    }
}
impl SYNCUSER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCUSER0_A> {
        match self.bits {
            0 => Some(SYNCUSER0_A::OFF),
            1 => Some(SYNCUSER0_A::SYNCCH0),
            2 => Some(SYNCUSER0_A::SYNCCH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYNCUSER0_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == SYNCUSER0_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `SYNCCH1`"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == SYNCUSER0_A::SYNCCH1
    }
}
#[doc = "Field `SYNCUSER0` writer - Synchronous User Ch 0 Input Selection - TCA0"]
pub type SYNCUSER0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SYNCUSER0_SPEC, u8, SYNCUSER0_A, 8, O>;
impl<'a, const O: u8> SYNCUSER0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SYNCUSER0_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(SYNCUSER0_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut W {
        self.variant(SYNCUSER0_A::SYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronous User Ch 0 Input Selection - TCA0"]
    #[inline(always)]
    pub fn syncuser0(&self) -> SYNCUSER0_R {
        SYNCUSER0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous User Ch 0 Input Selection - TCA0"]
    #[inline(always)]
    #[must_use]
    pub fn syncuser0(&mut self) -> SYNCUSER0_W<0> {
        SYNCUSER0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous User Ch 0 Input Selection - TCA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncuser0](index.html) module"]
pub struct SYNCUSER0_SPEC;
impl crate::RegisterSpec for SYNCUSER0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syncuser0::R](R) reader structure"]
impl crate::Readable for SYNCUSER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncuser0::W](W) writer structure"]
impl crate::Writable for SYNCUSER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCUSER0 to value 0"]
impl crate::Resettable for SYNCUSER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
