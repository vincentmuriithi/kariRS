#[doc = "Register `SERNUM%s` reader"]
pub struct R(crate::R<SERNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum](index.html) module"]
pub struct SERNUM_SPEC;
impl crate::RegisterSpec for SERNUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum::R](R) reader structure"]
impl crate::Readable for SERNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM%s to value 0"]
impl crate::Resettable for SERNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
