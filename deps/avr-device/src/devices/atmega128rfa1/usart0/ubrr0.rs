#[doc = "Register `UBRR0` reader"]
pub struct R(crate::R<UBRR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UBRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UBRR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UBRR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UBRR0` writer"]
pub struct W(crate::W<UBRR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UBRR0_SPEC>;
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
impl From<crate::W<UBRR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UBRR0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USART0 Baud Rate Register Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ubrr0](index.html) module"]
pub struct UBRR0_SPEC;
impl crate::RegisterSpec for UBRR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ubrr0::R](R) reader structure"]
impl crate::Readable for UBRR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ubrr0::W](W) writer structure"]
impl crate::Writable for UBRR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UBRR0 to value 0"]
impl crate::Resettable for UBRR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
