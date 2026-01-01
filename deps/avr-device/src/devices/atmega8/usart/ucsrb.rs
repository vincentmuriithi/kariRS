#[doc = "Register `UCSRB` reader"]
pub struct R(crate::R<UCSRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSRB` writer"]
pub struct W(crate::W<UCSRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSRB_SPEC>;
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
impl From<crate::W<UCSRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXB8` reader - Transmit Data Bit 8"]
pub type TXB8_R = crate::BitReader<bool>;
#[doc = "Field `TXB8` writer - Transmit Data Bit 8"]
pub type TXB8_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
#[doc = "Field `RXB8` reader - Receive Data Bit 8"]
pub type RXB8_R = crate::BitReader<bool>;
#[doc = "Field `UCSZ2` reader - Character Size"]
pub type UCSZ2_R = crate::BitReader<bool>;
#[doc = "Field `UCSZ2` writer - Character Size"]
pub type UCSZ2_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
#[doc = "Field `UDRIE` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE_R = crate::BitReader<bool>;
#[doc = "Field `UDRIE` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
#[doc = "Field `TXCIE` reader - TX Complete Interrupt Enable"]
pub type TXCIE_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE` writer - TX Complete Interrupt Enable"]
pub type TXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
#[doc = "Field `RXCIE` reader - RX Complete Interrupt Enable"]
pub type RXCIE_R = crate::BitReader<bool>;
#[doc = "Field `RXCIE` writer - RX Complete Interrupt Enable"]
pub type RXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb8(&self) -> TXB8_R {
        TXB8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb8(&self) -> RXB8_R {
        RXB8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    pub fn ucsz2(&self) -> UCSZ2_R {
        UCSZ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&self) -> RXCIE_R {
        RXCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb8(&mut self) -> TXB8_W<0> {
        TXB8_W::new(self)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz2(&mut self) -> UCSZ2_W<2> {
        UCSZ2_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<3> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<4> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<5> {
        UDRIE_W::new(self)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<6> {
        TXCIE_W::new(self)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie(&mut self) -> RXCIE_W<7> {
        RXCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsrb](index.html) module"]
pub struct UCSRB_SPEC;
impl crate::RegisterSpec for UCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsrb::R](R) reader structure"]
impl crate::Readable for UCSRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsrb::W](W) writer structure"]
impl crate::Writable for UCSRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSRB to value 0"]
impl crate::Resettable for UCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
