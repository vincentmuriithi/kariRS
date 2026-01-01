#[doc = "Register `TEMPSENSE1` reader"]
pub struct R(crate::R<TEMPSENSE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPSENSE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPSENSE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPSENSE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Temperature Sensor Calibration: Offset\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense1](index.html) module"]
pub struct TEMPSENSE1_SPEC;
impl crate::RegisterSpec for TEMPSENSE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tempsense1::R](R) reader structure"]
impl crate::Readable for TEMPSENSE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEMPSENSE1 to value 0"]
impl crate::Resettable for TEMPSENSE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
