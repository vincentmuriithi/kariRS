#[doc = "Register `BOOTEND` reader"]
pub struct R(crate::R<BOOTEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Boot Section End\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootend](index.html) module"]
pub struct BOOTEND_SPEC;
impl crate::RegisterSpec for BOOTEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bootend::R](R) reader structure"]
impl crate::Readable for BOOTEND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BOOTEND to value 0"]
impl crate::Resettable for BOOTEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
