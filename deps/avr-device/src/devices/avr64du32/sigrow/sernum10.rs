#[doc = "Register `SERNUM10` reader"]
pub struct R(crate::R<SERNUM10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "YPOS0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum10](index.html) module"]
pub struct SERNUM10_SPEC;
impl crate::RegisterSpec for SERNUM10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum10::R](R) reader structure"]
impl crate::Readable for SERNUM10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM10 to value 0"]
impl crate::Resettable for SERNUM10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
