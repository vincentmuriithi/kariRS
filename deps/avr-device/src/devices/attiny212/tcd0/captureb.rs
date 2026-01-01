#[doc = "Register `CAPTUREB` reader"]
pub struct R(crate::R<CAPTUREB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTUREB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTUREB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTUREB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Capture B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captureb](index.html) module"]
pub struct CAPTUREB_SPEC;
impl crate::RegisterSpec for CAPTUREB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [captureb::R](R) reader structure"]
impl crate::Readable for CAPTUREB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPTUREB to value 0"]
impl crate::Resettable for CAPTUREB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
