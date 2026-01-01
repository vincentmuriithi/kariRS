#[doc = "Register `CLKSEL` reader"]
pub struct R(crate::R<CLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSEL` writer"]
pub struct W(crate::W<CLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSEL_SPEC>;
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
impl From<crate::W<CLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Internal 32kHz OSC"]
    INT32K = 0,
    #[doc = "1: Internal 1kHz OSC"]
    INT1K = 1,
    #[doc = "3: External Clock"]
    EXTCLK = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::INT32K),
            1 => Some(CLKSEL_A::INT1K),
            3 => Some(CLKSEL_A::EXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT32K`"]
    #[inline(always)]
    pub fn is_int32k(&self) -> bool {
        *self == CLKSEL_A::INT32K
    }
    #[doc = "Checks if the value of the field is `INT1K`"]
    #[inline(always)]
    pub fn is_int1k(&self) -> bool {
        *self == CLKSEL_A::INT1K
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == CLKSEL_A::EXTCLK
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLKSEL_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Internal 32kHz OSC"]
    #[inline(always)]
    pub fn int32k(self) -> &'a mut W {
        self.variant(CLKSEL_A::INT32K)
    }
    #[doc = "Internal 1kHz OSC"]
    #[inline(always)]
    pub fn int1k(self) -> &'a mut W {
        self.variant(CLKSEL_A::INT1K)
    }
    #[doc = "External Clock"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EXTCLK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel](index.html) module"]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clksel::R](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksel::W](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
