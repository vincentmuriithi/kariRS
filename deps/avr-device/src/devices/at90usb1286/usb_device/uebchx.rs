#[doc = "Register `UEBCHX` reader"]
pub struct R(crate::R<UEBCHX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEBCHX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEBCHX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEBCHX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEBCHX` writer"]
pub struct W(crate::W<UEBCHX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEBCHX_SPEC>;
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
impl From<crate::W<UEBCHX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEBCHX_SPEC>) -> Self {
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
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uebchx](index.html) module"]
pub struct UEBCHX_SPEC;
impl crate::RegisterSpec for UEBCHX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uebchx::R](R) reader structure"]
impl crate::Readable for UEBCHX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uebchx::W](W) writer structure"]
impl crate::Writable for UEBCHX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEBCHX to value 0"]
impl crate::Resettable for UEBCHX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
