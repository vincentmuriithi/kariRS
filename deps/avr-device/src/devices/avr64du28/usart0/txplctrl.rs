#[doc = "Register `TXPLCTRL` reader"]
pub struct R(crate::R<TXPLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPLCTRL` writer"]
pub struct W(crate::W<TXPLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPLCTRL_SPEC>;
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
impl From<crate::W<TXPLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPL` reader - Transmit pulse length"]
pub type TXPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXPL` writer - Transmit pulse length"]
pub type TXPL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TXPLCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn txpl(&self) -> TXPL_R {
        TXPL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit pulse length"]
    #[inline(always)]
    #[must_use]
    pub fn txpl(&mut self) -> TXPL_W<0> {
        TXPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "IRCOM Transmitter Pulse Length Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txplctrl](index.html) module"]
pub struct TXPLCTRL_SPEC;
impl crate::RegisterSpec for TXPLCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txplctrl::R](R) reader structure"]
impl crate::Readable for TXPLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txplctrl::W](W) writer structure"]
impl crate::Writable for TXPLCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPLCTRL to value 0"]
impl crate::Resettable for TXPLCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
