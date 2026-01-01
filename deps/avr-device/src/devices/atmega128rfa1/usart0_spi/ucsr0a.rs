#[doc = "Register `UCSR0A` reader"]
pub struct R(crate::R<UCSR0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR0A` writer"]
pub struct W(crate::W<UCSR0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR0A_SPEC>;
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
impl From<crate::W<UCSR0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDRE0` reader - USART Data Register Empty"]
pub type UDRE0_R = crate::BitReader<bool>;
#[doc = "Field `UDRE0` writer - USART Data Register Empty"]
pub type UDRE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0A_SPEC, bool, O>;
#[doc = "Field `TXC0` reader - USART Transmit Complete"]
pub type TXC0_R = crate::BitReader<bool>;
#[doc = "Field `TXC0` writer - USART Transmit Complete"]
pub type TXC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0A_SPEC, bool, O>;
#[doc = "Field `RXC0` reader - USART Receive Complete"]
pub type RXC0_R = crate::BitReader<bool>;
#[doc = "Field `RXC0` writer - USART Receive Complete"]
pub type RXC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0A_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre0(&self) -> UDRE0_R {
        UDRE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc0(&self) -> TXC0_R {
        TXC0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc0(&self) -> RXC0_R {
        RXC0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    #[must_use]
    pub fn udre0(&mut self) -> UDRE0_W<5> {
        UDRE0_W::new(self)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc0(&mut self) -> TXC0_W<6> {
        TXC0_W::new(self)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rxc0(&mut self) -> RXC0_W<7> {
        RXC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART0 MSPIM Control and Status Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr0a](index.html) module"]
pub struct UCSR0A_SPEC;
impl crate::RegisterSpec for UCSR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr0a::R](R) reader structure"]
impl crate::Readable for UCSR0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr0a::W](W) writer structure"]
impl crate::Writable for UCSR0A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0A to value 0"]
impl crate::Resettable for UCSR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
