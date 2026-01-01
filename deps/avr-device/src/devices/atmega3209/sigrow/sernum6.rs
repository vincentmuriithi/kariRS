#[doc = "Register `SERNUM6` reader"]
pub struct R(crate::R<SERNUM6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Byte 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum6](index.html) module"]
pub struct SERNUM6_SPEC;
impl crate::RegisterSpec for SERNUM6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum6::R](R) reader structure"]
impl crate::Readable for SERNUM6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM6 to value 0"]
impl crate::Resettable for SERNUM6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
