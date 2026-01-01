#[doc = "Register `SERNUM1` reader"]
pub struct R(crate::R<SERNUM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "LOTNUM1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum1](index.html) module"]
pub struct SERNUM1_SPEC;
impl crate::RegisterSpec for SERNUM1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum1::R](R) reader structure"]
impl crate::Readable for SERNUM1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM1 to value 0"]
impl crate::Resettable for SERNUM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
