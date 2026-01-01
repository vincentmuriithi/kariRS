#[doc = "Register `SERNUM13` reader"]
pub struct R(crate::R<SERNUM13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RES1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum13](index.html) module"]
pub struct SERNUM13_SPEC;
impl crate::RegisterSpec for SERNUM13_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum13::R](R) reader structure"]
impl crate::Readable for SERNUM13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM13 to value 0"]
impl crate::Resettable for SERNUM13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
