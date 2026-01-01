#[doc = "Register `DEVICEID2` reader"]
pub struct R(crate::R<DEVICEID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device ID Byte 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid2](index.html) module"]
pub struct DEVICEID2_SPEC;
impl crate::RegisterSpec for DEVICEID2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [deviceid2::R](R) reader structure"]
impl crate::Readable for DEVICEID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEID2 to value 0"]
impl crate::Resettable for DEVICEID2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
