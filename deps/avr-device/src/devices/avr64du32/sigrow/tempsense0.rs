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
#[doc = "Field `TEMPSENSE0` reader - Temperature Calibration 0"]
pub type TEMPSENSE0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Calibration 0"]
    #[inline(always)]
    pub fn tempsense0(&self) -> TEMPSENSE0_R {
        TEMPSENSE0_R::new(self.bits)
    }
}
#[doc = "Temperature Calibration 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0](index.html) module"]
pub struct TEMPSENSE0_SPEC;
impl crate::RegisterSpec for TEMPSENSE0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tempsense0::R](R) reader structure"]
impl crate::Readable for TEMPSENSE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEMPSENSE0 to value 0"]
impl crate::Resettable for TEMPSENSE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
