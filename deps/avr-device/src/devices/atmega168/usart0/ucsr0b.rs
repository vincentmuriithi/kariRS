#[doc = "Register `UCSR0B` reader"]
pub struct R(crate::R<UCSR0B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR0B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR0B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR0B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR0B` writer"]
pub struct W(crate::W<UCSR0B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR0B_SPEC>;
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
impl From<crate::W<UCSR0B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR0B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXB80` reader - Transmit Data Bit 8"]
pub type TXB80_R = crate::BitReader<bool>;
#[doc = "Field `TXB80` writer - Transmit Data Bit 8"]
pub type TXB80_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
#[doc = "Field `RXB80` reader - Receive Data Bit 8"]
pub type RXB80_R = crate::BitReader<bool>;
#[doc = "Field `UCSZ02` reader - Character Size - together with UCSZ0 in UCSR0C"]
pub type UCSZ02_R = crate::BitReader<bool>;
#[doc = "Field `UCSZ02` writer - Character Size - together with UCSZ0 in UCSR0C"]
pub type UCSZ02_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
#[doc = "Field `TXEN0` reader - Transmitter Enable"]
pub type TXEN0_R = crate::BitReader<bool>;
#[doc = "Field `TXEN0` writer - Transmitter Enable"]
pub type TXEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
#[doc = "Field `RXEN0` reader - Receiver Enable"]
pub type RXEN0_R = crate::BitReader<bool>;
#[doc = "Field `RXEN0` writer - Receiver Enable"]
pub type RXEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
#[doc = "Field `UDRIE0` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE0_R = crate::BitReader<bool>;
#[doc = "Field `UDRIE0` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
#[doc = "Field `TXCIE0` reader - TX Complete Interrupt Enable"]
pub type TXCIE0_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE0` writer - TX Complete Interrupt Enable"]
pub type TXCIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
#[doc = "Field `RXCIE0` reader - RX Complete Interrupt Enable"]
pub type RXCIE0_R = crate::BitReader<bool>;
#[doc = "Field `RXCIE0` writer - RX Complete Interrupt Enable"]
pub type RXCIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0B_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb80(&self) -> TXB80_R {
        TXB80_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb80(&self) -> RXB80_R {
        RXB80_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size - together with UCSZ0 in UCSR0C"]
    #[inline(always)]
    pub fn ucsz02(&self) -> UCSZ02_R {
        UCSZ02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen0(&self) -> TXEN0_R {
        TXEN0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen0(&self) -> RXEN0_R {
        RXEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie0(&self) -> UDRIE0_R {
        UDRIE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie0(&self) -> TXCIE0_R {
        TXCIE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie0(&self) -> RXCIE0_R {
        RXCIE0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb80(&mut self) -> TXB80_W<0> {
        TXB80_W::new(self)
    }
    #[doc = "Bit 2 - Character Size - together with UCSZ0 in UCSR0C"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz02(&mut self) -> UCSZ02_W<2> {
        UCSZ02_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen0(&mut self) -> TXEN0_W<3> {
        TXEN0_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen0(&mut self) -> RXEN0_W<4> {
        RXEN0_W::new(self)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie0(&mut self) -> UDRIE0_W<5> {
        UDRIE0_W::new(self)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie0(&mut self) -> TXCIE0_W<6> {
        TXCIE0_W::new(self)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie0(&mut self) -> RXCIE0_W<7> {
        RXCIE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr0b](index.html) module"]
pub struct UCSR0B_SPEC;
impl crate::RegisterSpec for UCSR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr0b::R](R) reader structure"]
impl crate::Readable for UCSR0B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr0b::W](W) writer structure"]
impl crate::Writable for UCSR0B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0B to value 0"]
impl crate::Resettable for UCSR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
