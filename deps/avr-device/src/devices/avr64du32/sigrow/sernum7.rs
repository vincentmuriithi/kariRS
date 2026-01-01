#[doc = "Register `SERNUM7` reader"]
pub struct R(crate::R<SERNUM7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SCRIBE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum7](index.html) module"]
pub struct SERNUM7_SPEC;
impl crate::RegisterSpec for SERNUM7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum7::R](R) reader structure"]
impl crate::Readable for SERNUM7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM7 to value 0"]
impl crate::Resettable for SERNUM7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
