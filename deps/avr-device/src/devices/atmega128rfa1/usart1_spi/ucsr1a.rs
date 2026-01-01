#[doc = "Register `UCSR1A` reader"]
pub struct R(crate::R<UCSR1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR1A` writer"]
pub struct W(crate::W<UCSR1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR1A_SPEC>;
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
impl From<crate::W<UCSR1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDRE1` reader - USART Data Register Empty"]
pub type UDRE1_R = crate::BitReader<bool>;
#[doc = "Field `UDRE1` writer - USART Data Register Empty"]
pub type UDRE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1A_SPEC, bool, O>;
#[doc = "Field `TXC1` reader - USART Transmit Complete"]
pub type TXC1_R = crate::BitReader<bool>;
#[doc = "Field `TXC1` writer - USART Transmit Complete"]
pub type TXC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1A_SPEC, bool, O>;
#[doc = "Field `RXC1` reader - USART Receive Complete"]
pub type RXC1_R = crate::BitReader<bool>;
#[doc = "Field `RXC1` writer - USART Receive Complete"]
pub type RXC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1A_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre1(&self) -> UDRE1_R {
        UDRE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc1(&self) -> TXC1_R {
        TXC1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc1(&self) -> RXC1_R {
        RXC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    #[must_use]
    pub fn udre1(&mut self) -> UDRE1_W<5> {
        UDRE1_W::new(self)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc1(&mut self) -> TXC1_W<6> {
        TXC1_W::new(self)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rxc1(&mut self) -> RXC1_W<7> {
        RXC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART1 MSPIM Control and Status Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr1a](index.html) module"]
pub struct UCSR1A_SPEC;
impl crate::RegisterSpec for UCSR1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr1a::R](R) reader structure"]
impl crate::Readable for UCSR1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr1a::W](W) writer structure"]
impl crate::Writable for UCSR1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1A to value 0"]
impl crate::Resettable for UCSR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
