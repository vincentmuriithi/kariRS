#[doc = "Register `SERNUM11` reader"]
pub struct R(crate::R<SERNUM11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "YPOS1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum11](index.html) module"]
pub struct SERNUM11_SPEC;
impl crate::RegisterSpec for SERNUM11_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum11::R](R) reader structure"]
impl crate::Readable for SERNUM11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM11 to value 0"]
impl crate::Resettable for SERNUM11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
