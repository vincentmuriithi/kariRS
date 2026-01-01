#[doc = "Register `CAPTUREA` reader"]
pub struct R(crate::R<CAPTUREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTUREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTUREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTUREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Capture A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturea](index.html) module"]
pub struct CAPTUREA_SPEC;
impl crate::RegisterSpec for CAPTUREA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [capturea::R](R) reader structure"]
impl crate::Readable for CAPTUREA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPTUREA to value 0"]
impl crate::Resettable for CAPTUREA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
