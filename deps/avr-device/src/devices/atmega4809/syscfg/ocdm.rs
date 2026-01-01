#[doc = "Register `OCDM` reader"]
pub struct R(crate::R<OCDM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCDM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCDM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCDM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCDM` writer"]
pub struct W(crate::W<OCDM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCDM_SPEC>;
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
impl From<crate::W<OCDM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCDM_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "OCD Message Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocdm](index.html) module"]
pub struct OCDM_SPEC;
impl crate::RegisterSpec for OCDM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ocdm::R](R) reader structure"]
impl crate::Readable for OCDM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocdm::W](W) writer structure"]
impl crate::Writable for OCDM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDM to value 0"]
impl crate::Resettable for OCDM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
