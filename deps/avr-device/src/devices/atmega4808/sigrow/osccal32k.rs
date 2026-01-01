#[doc = "Register `OSCCAL32K` reader"]
pub struct R(crate::R<OSCCAL32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCAL32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCAL32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCAL32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Oscillator Calibration for 32kHz ULP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccal32k](index.html) module"]
pub struct OSCCAL32K_SPEC;
impl crate::RegisterSpec for OSCCAL32K_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osccal32k::R](R) reader structure"]
impl crate::Readable for OSCCAL32K_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCCAL32K to value 0"]
impl crate::Resettable for OSCCAL32K_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
