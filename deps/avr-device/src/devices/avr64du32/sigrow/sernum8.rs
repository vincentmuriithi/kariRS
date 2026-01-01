#[doc = "Register `SERNUM8` reader"]
pub struct R(crate::R<SERNUM8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "XPOS0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum8](index.html) module"]
pub struct SERNUM8_SPEC;
impl crate::RegisterSpec for SERNUM8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum8::R](R) reader structure"]
impl crate::Readable for SERNUM8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM8 to value 0"]
impl crate::Resettable for SERNUM8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
