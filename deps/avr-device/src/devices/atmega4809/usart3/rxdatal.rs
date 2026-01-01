#[doc = "Register `RXDATAL` reader"]
pub struct R(crate::R<RXDATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - RX Data"]
pub type DATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Receive Data Low Byte\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdatal](index.html) module"]
pub struct RXDATAL_SPEC;
impl crate::RegisterSpec for RXDATAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxdatal::R](R) reader structure"]
impl crate::Readable for RXDATAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAL to value 0"]
impl crate::Resettable for RXDATAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
