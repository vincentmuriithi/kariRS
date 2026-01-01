#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - ADC Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - ADC Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `FREERUN` reader - ADC Freerun mode"]
pub type FREERUN_R = crate::BitReader<bool>;
#[doc = "Field `FREERUN` writer - ADC Freerun mode"]
pub type FREERUN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RESSEL` reader - ADC Resolution"]
pub type RESSEL_R = crate::BitReader<RESSEL_A>;
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESSEL_A {
    #[doc = "0: 10-bit mode"]
    _10BIT = 0,
    #[doc = "1: 8-bit mode"]
    _8BIT = 1,
}
impl From<RESSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSEL_A {
        match self.bits {
            false => RESSEL_A::_10BIT,
            true => RESSEL_A::_8BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RESSEL_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RESSEL_A::_8BIT
    }
}
#[doc = "Field `RESSEL` writer - ADC Resolution"]
pub type RESSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, RESSEL_A, O>;
impl<'a, const O: u8> RESSEL_W<'a, O> {
    #[doc = "10-bit mode"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_10BIT)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_8BIT)
    }
}
#[doc = "Field `RUNSTBY` reader - Run standby mode"]
pub type RUNSTBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTBY` writer - Run standby mode"]
pub type RUNSTBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Freerun mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Run standby mode"]
    #[inline(always)]
    pub fn runstby(&self) -> RUNSTBY_R {
        RUNSTBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - ADC Freerun mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<1> {
        FREERUN_W::new(self)
    }
    #[doc = "Bit 2 - ADC Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<2> {
        RESSEL_W::new(self)
    }
    #[doc = "Bit 7 - Run standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstby(&mut self) -> RUNSTBY_W<7> {
        RUNSTBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
