#[doc = "Register `RXPLCTRL` reader"]
pub struct R(crate::R<RXPLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXPLCTRL` writer"]
pub struct W(crate::W<RXPLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXPLCTRL_SPEC>;
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
impl From<crate::W<RXPLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXPLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPL` reader - Receiver Pulse Lenght"]
pub type RXPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPL` writer - Receiver Pulse Lenght"]
pub type RXPL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RXPLCTRL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Receiver Pulse Lenght"]
    #[inline(always)]
    pub fn rxpl(&self) -> RXPL_R {
        RXPL_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Receiver Pulse Lenght"]
    #[inline(always)]
    #[must_use]
    pub fn rxpl(&mut self) -> RXPL_W<0> {
        RXPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRCOM Receiver Pulse Length Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxplctrl](index.html) module"]
pub struct RXPLCTRL_SPEC;
impl crate::RegisterSpec for RXPLCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxplctrl::R](R) reader structure"]
impl crate::Readable for RXPLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxplctrl::W](W) writer structure"]
impl crate::Writable for RXPLCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXPLCTRL to value 0"]
impl crate::Resettable for RXPLCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
