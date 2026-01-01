#[doc = "Register `SERNUM3` reader"]
pub struct R(crate::R<SERNUM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Byte 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum3](index.html) module"]
pub struct SERNUM3_SPEC;
impl crate::RegisterSpec for SERNUM3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum3::R](R) reader structure"]
impl crate::Readable for SERNUM3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM3 to value 0"]
impl crate::Resettable for SERNUM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
