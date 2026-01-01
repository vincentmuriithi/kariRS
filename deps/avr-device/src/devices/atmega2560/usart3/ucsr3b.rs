#[doc = "Register `UCSR3B` reader"]
pub struct R(crate::R<UCSR3B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR3B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR3B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR3B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR3B` writer"]
pub struct W(crate::W<UCSR3B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR3B_SPEC>;
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
impl From<crate::W<UCSR3B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR3B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXB83` reader - Transmit Data Bit 8"]
pub type TXB83_R = crate::BitReader<bool>;
#[doc = "Field `TXB83` writer - Transmit Data Bit 8"]
pub type TXB83_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
#[doc = "Field `RXB83` reader - Receive Data Bit 8"]
pub type RXB83_R = crate::BitReader<bool>;
#[doc = "Field `UCSZ32` reader - Character Size"]
pub type UCSZ32_R = crate::BitReader<bool>;
#[doc = "Field `UCSZ32` writer - Character Size"]
pub type UCSZ32_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
#[doc = "Field `TXEN3` reader - Transmitter Enable"]
pub type TXEN3_R = crate::BitReader<bool>;
#[doc = "Field `TXEN3` writer - Transmitter Enable"]
pub type TXEN3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
#[doc = "Field `RXEN3` reader - Receiver Enable"]
pub type RXEN3_R = crate::BitReader<bool>;
#[doc = "Field `RXEN3` writer - Receiver Enable"]
pub type RXEN3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
#[doc = "Field `UDRIE3` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE3_R = crate::BitReader<bool>;
#[doc = "Field `UDRIE3` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
#[doc = "Field `TXCIE3` reader - TX Complete Interrupt Enable"]
pub type TXCIE3_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE3` writer - TX Complete Interrupt Enable"]
pub type TXCIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
#[doc = "Field `RXCIE3` reader - RX Complete Interrupt Enable"]
pub type RXCIE3_R = crate::BitReader<bool>;
#[doc = "Field `RXCIE3` writer - RX Complete Interrupt Enable"]
pub type RXCIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR3B_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb83(&self) -> TXB83_R {
        TXB83_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb83(&self) -> RXB83_R {
        RXB83_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    pub fn ucsz32(&self) -> UCSZ32_R {
        UCSZ32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen3(&self) -> TXEN3_R {
        TXEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen3(&self) -> RXEN3_R {
        RXEN3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie3(&self) -> UDRIE3_R {
        UDRIE3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie3(&self) -> TXCIE3_R {
        TXCIE3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie3(&self) -> RXCIE3_R {
        RXCIE3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb83(&mut self) -> TXB83_W<0> {
        TXB83_W::new(self)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz32(&mut self) -> UCSZ32_W<2> {
        UCSZ32_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen3(&mut self) -> TXEN3_W<3> {
        TXEN3_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen3(&mut self) -> RXEN3_W<4> {
        RXEN3_W::new(self)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie3(&mut self) -> UDRIE3_W<5> {
        UDRIE3_W::new(self)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie3(&mut self) -> TXCIE3_W<6> {
        TXCIE3_W::new(self)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie3(&mut self) -> RXCIE3_W<7> {
        RXCIE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr3b](index.html) module"]
pub struct UCSR3B_SPEC;
impl crate::RegisterSpec for UCSR3B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr3b::R](R) reader structure"]
impl crate::Readable for UCSR3B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr3b::W](W) writer structure"]
impl crate::Writable for UCSR3B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR3B to value 0"]
impl crate::Resettable for UCSR3B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
