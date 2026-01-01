#[doc = "Register `SERNUM15` reader"]
pub struct R(crate::R<SERNUM15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RES3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum15](index.html) module"]
pub struct SERNUM15_SPEC;
impl crate::RegisterSpec for SERNUM15_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum15::R](R) reader structure"]
impl crate::Readable for SERNUM15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM15 to value 0"]
impl crate::Resettable for SERNUM15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
