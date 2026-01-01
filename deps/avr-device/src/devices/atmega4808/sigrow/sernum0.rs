#[doc = "Register `SERNUM0` reader"]
pub struct R(crate::R<SERNUM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Byte 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum0](index.html) module"]
pub struct SERNUM0_SPEC;
impl crate::RegisterSpec for SERNUM0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum0::R](R) reader structure"]
impl crate::Readable for SERNUM0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM0 to value 0"]
impl crate::Resettable for SERNUM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
