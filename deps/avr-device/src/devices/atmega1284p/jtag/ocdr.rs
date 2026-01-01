#[doc = "Register `OCDR` reader"]
pub struct R(crate::R<OCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCDR` writer"]
pub struct W(crate::W<OCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCDR_SPEC>;
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
impl From<crate::W<OCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCDR_SPEC>) -> Self {
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
#[doc = "On-Chip Debug Related Register in I/O Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocdr](index.html) module"]
pub struct OCDR_SPEC;
impl crate::RegisterSpec for OCDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ocdr::R](R) reader structure"]
impl crate::Readable for OCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocdr::W](W) writer structure"]
impl crate::Writable for OCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDR to value 0"]
impl crate::Resettable for OCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
