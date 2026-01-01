#[doc = "Register `REGCR` reader"]
pub struct R(crate::R<REGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGCR` writer"]
pub struct W(crate::W<REGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGCR_SPEC>;
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
impl From<crate::W<REGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGDIS` reader - Regulator Disable"]
pub type REGDIS_R = crate::BitReader<bool>;
#[doc = "Field `REGDIS` writer - Regulator Disable"]
pub type REGDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, REGCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Regulator Disable"]
    #[inline(always)]
    pub fn regdis(&self) -> REGDIS_R {
        REGDIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Regulator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regdis(&mut self) -> REGDIS_W<0> {
        REGDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regcr](index.html) module"]
pub struct REGCR_SPEC;
impl crate::RegisterSpec for REGCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [regcr::R](R) reader structure"]
impl crate::Readable for REGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regcr::W](W) writer structure"]
impl crate::Writable for REGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGCR to value 0"]
impl crate::Resettable for REGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
