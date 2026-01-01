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
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BUFOVF_R = crate::BitReader<bool>;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BUFOVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `SSIF` reader - Client Select Trigger Interrupt Flag"]
pub type SSIF_R = crate::BitReader<bool>;
#[doc = "Field `SSIF` writer - Client Select Trigger Interrupt Flag"]
pub type SSIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `DREIF` reader - Data Register Empty Interrupt Flag"]
pub type DREIF_R = crate::BitReader<bool>;
#[doc = "Field `DREIF` writer - Data Register Empty Interrupt Flag"]
pub type DREIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `TXCIF` reader - Transfer Complete Interrupt Flag"]
pub type TXCIF_R = crate::BitReader<bool>;
#[doc = "Field `TXCIF` writer - Transfer Complete Interrupt Flag"]
pub type TXCIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `WRCOL` reader - Write Collision"]
pub type WRCOL_R = crate::BitReader<bool>;
#[doc = "Field `WRCOL` writer - Write Collision"]
pub type WRCOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `IF` reader - Interrupt Flag"]
pub type IF_R = crate::BitReader<bool>;
#[doc = "Field `IF` writer - Interrupt Flag"]
pub type IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `RXCIF` reader - Receive Complete Interrupt Flag"]
pub type RXCIF_R = crate::BitReader<bool>;
#[doc = "Field `RXCIF` writer - Receive Complete Interrupt Flag"]
pub type RXCIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BUFOVF_R {
        BUFOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Flag"]
    #[inline(always)]
    pub fn ssif(&self) -> SSIF_R {
        SSIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Flag"]
    #[inline(always)]
    pub fn dreif(&self) -> DREIF_R {
        DREIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txcif(&self) -> TXCIF_R {
        TXCIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Collision"]
    #[inline(always)]
    pub fn wrcol(&self) -> WRCOL_R {
        WRCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn rxcif(&self) -> RXCIF_R {
        RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn bufovf(&mut self) -> BUFOVF_W<0> {
        BUFOVF_W::new(self)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssif(&mut self) -> SSIF_W<4> {
        SSIF_W::new(self)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dreif(&mut self) -> DREIF_W<5> {
        DREIF_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txcif(&mut self) -> TXCIF_W<6> {
        TXCIF_W::new(self)
    }
    #[doc = "Bit 6 - Write Collision"]
    #[inline(always)]
    #[must_use]
    pub fn wrcol(&mut self) -> WRCOL_W<6> {
        WRCOL_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn if_(&mut self) -> IF_W<7> {
        IF_W::new(self)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxcif(&mut self) -> RXCIF_W<7> {
        RXCIF_W::new(self)
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
