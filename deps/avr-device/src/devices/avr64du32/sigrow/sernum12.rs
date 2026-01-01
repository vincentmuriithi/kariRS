#[doc = "Register `SERNUM12` reader"]
pub struct R(crate::R<SERNUM12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RES0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum12](index.html) module"]
pub struct SERNUM12_SPEC;
impl crate::RegisterSpec for SERNUM12_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum12::R](R) reader structure"]
impl crate::Readable for SERNUM12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM12 to value 0"]
impl crate::Resettable for SERNUM12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
