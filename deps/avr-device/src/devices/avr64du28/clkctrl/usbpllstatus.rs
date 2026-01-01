#[doc = "Register `USBPLLSTATUS` reader"]
pub struct R(crate::R<USBPLLSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLLS` reader - PLL Stable"]
pub type PLLS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PLL Stable"]
    #[inline(always)]
    pub fn plls(&self) -> PLLS_R {
        PLLS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllstatus](index.html) module"]
pub struct USBPLLSTATUS_SPEC;
impl crate::RegisterSpec for USBPLLSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbpllstatus::R](R) reader structure"]
impl crate::Readable for USBPLLSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBPLLSTATUS to value 0"]
impl crate::Resettable for USBPLLSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
