#[doc = "Register `APPEND` reader"]
pub struct R(crate::R<APPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Application Code Section End\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [append](index.html) module"]
pub struct APPEND_SPEC;
impl crate::RegisterSpec for APPEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [append::R](R) reader structure"]
impl crate::Readable for APPEND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APPEND to value 0"]
impl crate::Resettable for APPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
