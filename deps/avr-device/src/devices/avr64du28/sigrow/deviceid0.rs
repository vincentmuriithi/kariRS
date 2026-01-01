#[doc = "Register `DEVICEID0` reader"]
pub struct R(crate::R<DEVICEID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device ID Byte 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid0](index.html) module"]
pub struct DEVICEID0_SPEC;
impl crate::RegisterSpec for DEVICEID0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [deviceid0::R](R) reader structure"]
impl crate::Readable for DEVICEID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEID0 to value 0"]
impl crate::Resettable for DEVICEID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
