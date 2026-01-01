#[doc = "Register `INTFLAGS` reader"]
pub struct R(crate::R<INTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGS` writer"]
pub struct W(crate::W<INTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGS_SPEC>;
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
impl From<crate::W<INTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFERED_BUFOVF` reader - Buffer Overflow"]
pub type BUFFERED_BUFOVF_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERED_BUFOVF` writer - Buffer Overflow"]
pub type BUFFERED_BUFOVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `BUFFERED_SSIF` reader - Slave Select Trigger Interrupt Flag"]
pub type BUFFERED_SSIF_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERED_SSIF` writer - Slave Select Trigger Interrupt Flag"]
pub type BUFFERED_SSIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `BUFFERED_DREIF` reader - Data Register Empty Interrupt Flag"]
pub type BUFFERED_DREIF_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERED_DREIF` writer - Data Register Empty Interrupt Flag"]
pub type BUFFERED_DREIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `BUFFERED_TXCIF` reader - Transfer Complete Interrupt Flag"]
pub type BUFFERED_TXCIF_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERED_TXCIF` writer - Transfer Complete Interrupt Flag"]
pub type BUFFERED_TXCIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `DEFAULT_WRCOL` reader - Write Collision"]
pub type DEFAULT_WRCOL_R = crate::BitReader<bool>;
#[doc = "Field `DEFAULT_WRCOL` writer - Write Collision"]
pub type DEFAULT_WRCOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `BUFFERED_RXCIF` reader - Receive Complete Interrupt Flag"]
pub type BUFFERED_RXCIF_R = crate::BitReader<bool>;
#[doc = "Field `BUFFERED_RXCIF` writer - Receive Complete Interrupt Flag"]
pub type BUFFERED_RXCIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `DEFAULT_IF` reader - Interrupt Flag"]
pub type DEFAULT_IF_R = crate::BitReader<bool>;
#[doc = "Field `DEFAULT_IF` writer - Interrupt Flag"]
pub type DEFAULT_IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Buffer Overflow"]
    #[inline(always)]
    pub fn buffered_bufovf(&self) -> BUFFERED_BUFOVF_R {
        BUFFERED_BUFOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Trigger Interrupt Flag"]
    #[inline(always)]
    pub fn buffered_ssif(&self) -> BUFFERED_SSIF_R {
        BUFFERED_SSIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Flag"]
    #[inline(always)]
    pub fn buffered_dreif(&self) -> BUFFERED_DREIF_R {
        BUFFERED_DREIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Flag"]
    #[inline(always)]
    pub fn buffered_txcif(&self) -> BUFFERED_TXCIF_R {
        BUFFERED_TXCIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Collision"]
    #[inline(always)]
    pub fn default_wrcol(&self) -> DEFAULT_WRCOL_R {
        DEFAULT_WRCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn buffered_rxcif(&self) -> BUFFERED_RXCIF_R {
        BUFFERED_RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Flag"]
    #[inline(always)]
    pub fn default_if(&self) -> DEFAULT_IF_R {
        DEFAULT_IF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn buffered_bufovf(&mut self) -> BUFFERED_BUFOVF_W<0> {
        BUFFERED_BUFOVF_W::new(self)
    }
    #[doc = "Bit 4 - Slave Select Trigger Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buffered_ssif(&mut self) -> BUFFERED_SSIF_W<4> {
        BUFFERED_SSIF_W::new(self)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buffered_dreif(&mut self) -> BUFFERED_DREIF_W<5> {
        BUFFERED_DREIF_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buffered_txcif(&mut self) -> BUFFERED_TXCIF_W<6> {
        BUFFERED_TXCIF_W::new(self)
    }
    #[doc = "Bit 6 - Write Collision"]
    #[inline(always)]
    #[must_use]
    pub fn default_wrcol(&mut self) -> DEFAULT_WRCOL_W<6> {
        DEFAULT_WRCOL_W::new(self)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buffered_rxcif(&mut self) -> BUFFERED_RXCIF_W<7> {
        BUFFERED_RXCIF_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn default_if(&mut self) -> DEFAULT_IF_W<7> {
        DEFAULT_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflags](index.html) module"]
pub struct INTFLAGS_SPEC;
impl crate::RegisterSpec for INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflags::R](R) reader structure"]
impl crate::Readable for INTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflags::W](W) writer structure"]
impl crate::Writable for INTFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::Resettable for INTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
