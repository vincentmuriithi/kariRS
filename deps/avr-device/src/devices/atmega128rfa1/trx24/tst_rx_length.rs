#[doc = "Register `TST_RX_LENGTH` reader"]
pub struct R(crate::R<TST_RX_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_RX_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_RX_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_RX_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_RX_LENGTH` writer"]
pub struct W(crate::W<TST_RX_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_RX_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TST_RX_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_RX_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_LENGTH` reader - Received Frame Length"]
pub type RX_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_LENGTH` writer - Received Frame Length"]
pub type RX_LENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, TST_RX_LENGTH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Received Frame Length"]
    #[inline(always)]
    pub fn rx_length(&self) -> RX_LENGTH_R {
        RX_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Received Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_length(&mut self) -> RX_LENGTH_W<0> {
        RX_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Transceiver Received Frame Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_rx_length](index.html) module"]
pub struct TST_RX_LENGTH_SPEC;
impl crate::RegisterSpec for TST_RX_LENGTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tst_rx_length::R](R) reader structure"]
impl crate::Readable for TST_RX_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_rx_length::W](W) writer structure"]
impl crate::Writable for TST_RX_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST_RX_LENGTH to value 0"]
impl crate::Resettable for TST_RX_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
