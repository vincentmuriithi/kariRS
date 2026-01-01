#[doc = "Register `DEVICEID1` reader"]
pub struct R(crate::R<DEVICEID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device ID Byte 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid1](index.html) module"]
pub struct DEVICEID1_SPEC;
impl crate::RegisterSpec for DEVICEID1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [deviceid1::R](R) reader structure"]
impl crate::Readable for DEVICEID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEID1 to value 0"]
impl crate::Resettable for DEVICEID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
