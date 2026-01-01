#[doc = "Register `GTCCR` reader"]
pub struct R(crate::R<GTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCR` writer"]
pub struct W(crate::W<GTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCR_SPEC>;
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
impl From<crate::W<GTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSR10` reader - No Description."]
pub type PSR10_R = crate::BitReader<bool>;
#[doc = "Field `PSR10` writer - No Description."]
pub type PSR10_W<'a, const O: u8> = crate::BitWriter<'a, u8, GTCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn psr10(&self) -> PSR10_R {
        PSR10_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn psr10(&mut self) -> PSR10_W<0> {
        PSR10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Timer Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccr](index.html) module"]
pub struct GTCCR_SPEC;
impl crate::RegisterSpec for GTCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gtccr::R](R) reader structure"]
impl crate::Readable for GTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccr::W](W) writer structure"]
impl crate::Writable for GTCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCR to value 0"]
impl crate::Resettable for GTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
