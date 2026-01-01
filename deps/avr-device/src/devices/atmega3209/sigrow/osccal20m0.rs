#[doc = "Register `OSCCAL20M0` reader"]
pub struct R(crate::R<OSCCAL20M0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCAL20M0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCAL20M0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCAL20M0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Oscillator Calibration 20 MHz Byte 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccal20m0](index.html) module"]
pub struct OSCCAL20M0_SPEC;
impl crate::RegisterSpec for OSCCAL20M0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osccal20m0::R](R) reader structure"]
impl crate::Readable for OSCCAL20M0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCCAL20M0 to value 0"]
impl crate::Resettable for OSCCAL20M0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
