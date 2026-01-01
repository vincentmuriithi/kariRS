#[doc = "Register `TEMPSENSE0` reader"]
pub struct R(crate::R<TEMPSENSE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPSENSE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPSENSE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPSENSE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Temperature Sensor Calibration Byte 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0](index.html) module"]
pub struct TEMPSENSE0_SPEC;
impl crate::RegisterSpec for TEMPSENSE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tempsense0::R](R) reader structure"]
impl crate::Readable for TEMPSENSE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEMPSENSE0 to value 0"]
impl crate::Resettable for TEMPSENSE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
