#[doc = "Register `OSCCAL16M0` reader"]
pub struct R(crate::R<OSCCAL16M0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCAL16M0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCAL16M0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCAL16M0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Oscillator Calibration 16 MHz Byte 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccal16m0](index.html) module"]
pub struct OSCCAL16M0_SPEC;
impl crate::RegisterSpec for OSCCAL16M0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osccal16m0::R](R) reader structure"]
impl crate::Readable for OSCCAL16M0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCCAL16M0 to value 0"]
impl crate::Resettable for OSCCAL16M0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
