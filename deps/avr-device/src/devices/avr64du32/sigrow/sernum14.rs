#[doc = "Register `SERNUM14` reader"]
pub struct R(crate::R<SERNUM14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RES2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum14](index.html) module"]
pub struct SERNUM14_SPEC;
impl crate::RegisterSpec for SERNUM14_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum14::R](R) reader structure"]
impl crate::Readable for SERNUM14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM14 to value 0"]
impl crate::Resettable for SERNUM14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
