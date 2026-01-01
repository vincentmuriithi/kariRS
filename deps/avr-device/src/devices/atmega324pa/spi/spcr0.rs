#[doc = "Register `SPCR0` reader"]
pub struct R(crate::R<SPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR0` writer"]
pub struct W(crate::W<SPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR0_SPEC>;
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
impl From<crate::W<SPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPR00` reader - SPI Clock Rate Select 0"]
pub type SPR00_R = crate::BitReader<SPR00_A>;
#[doc = "SPI Clock Rate Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPR00_A {
    #[doc = "0: fosc/4"]
    FOSC_4 = 0,
    #[doc = "1: fosc/16"]
    FOSC_16 = 1,
}
impl From<SPR00_A> for bool {
    #[inline(always)]
    fn from(variant: SPR00_A) -> Self {
        variant as u8 != 0
    }
}
impl SPR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPR00_A {
        match self.bits {
            false => SPR00_A::FOSC_4,
            true => SPR00_A::FOSC_16,
        }
    }
    #[doc = "Checks if the value of the field is `FOSC_4`"]
    #[inline(always)]
    pub fn is_fosc_4(&self) -> bool {
        *self == SPR00_A::FOSC_4
    }
    #[doc = "Checks if the value of the field is `FOSC_16`"]
    #[inline(always)]
    pub fn is_fosc_16(&self) -> bool {
        *self == SPR00_A::FOSC_16
    }
}
#[doc = "Field `SPR00` writer - SPI Clock Rate Select 0"]
pub type SPR00_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, SPR00_A, O>;
impl<'a, const O: u8> SPR00_W<'a, O> {
    #[doc = "fosc/4"]
    #[inline(always)]
    pub fn fosc_4(self) -> &'a mut W {
        self.variant(SPR00_A::FOSC_4)
    }
    #[doc = "fosc/16"]
    #[inline(always)]
    pub fn fosc_16(self) -> &'a mut W {
        self.variant(SPR00_A::FOSC_16)
    }
}
#[doc = "Field `SPR10` reader - SPI Clock Rate Select 1"]
pub type SPR10_R = crate::BitReader<bool>;
#[doc = "Field `SPR10` writer - SPI Clock Rate Select 1"]
pub type SPR10_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
#[doc = "Field `CPHA0` reader - Clock Phase"]
pub type CPHA0_R = crate::BitReader<bool>;
#[doc = "Field `CPHA0` writer - Clock Phase"]
pub type CPHA0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
#[doc = "Field `CPOL0` reader - Clock polarity"]
pub type CPOL0_R = crate::BitReader<bool>;
#[doc = "Field `CPOL0` writer - Clock polarity"]
pub type CPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
#[doc = "Field `MSTR0` reader - Master/Slave Select"]
pub type MSTR0_R = crate::BitReader<bool>;
#[doc = "Field `MSTR0` writer - Master/Slave Select"]
pub type MSTR0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
#[doc = "Field `DORD0` reader - Data Order"]
pub type DORD0_R = crate::BitReader<bool>;
#[doc = "Field `DORD0` writer - Data Order"]
pub type DORD0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
#[doc = "Field `SPE0` reader - SPI Enable"]
pub type SPE0_R = crate::BitReader<bool>;
#[doc = "Field `SPE0` writer - SPI Enable"]
pub type SPE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
#[doc = "Field `SPIE0` reader - SPI Interrupt Enable"]
pub type SPIE0_R = crate::BitReader<bool>;
#[doc = "Field `SPIE0` writer - SPI Interrupt Enable"]
pub type SPIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI Clock Rate Select 0"]
    #[inline(always)]
    pub fn spr00(&self) -> SPR00_R {
        SPR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Clock Rate Select 1"]
    #[inline(always)]
    pub fn spr10(&self) -> SPR10_R {
        SPR10_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    pub fn cpha0(&self) -> CPHA0_R {
        CPHA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol0(&self) -> CPOL0_R {
        CPOL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    pub fn mstr0(&self) -> MSTR0_R {
        MSTR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    pub fn dord0(&self) -> DORD0_R {
        DORD0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    pub fn spe0(&self) -> SPE0_R {
        SPE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    pub fn spie0(&self) -> SPIE0_R {
        SPIE0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Clock Rate Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn spr00(&mut self) -> SPR00_W<0> {
        SPR00_W::new(self)
    }
    #[doc = "Bit 1 - SPI Clock Rate Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn spr10(&mut self) -> SPR10_W<1> {
        SPR10_W::new(self)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha0(&mut self) -> CPHA0_W<2> {
        CPHA0_W::new(self)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol0(&mut self) -> CPOL0_W<3> {
        CPOL0_W::new(self)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr0(&mut self) -> MSTR0_W<4> {
        MSTR0_W::new(self)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord0(&mut self) -> DORD0_W<5> {
        DORD0_W::new(self)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe0(&mut self) -> SPE0_W<6> {
        SPE0_W::new(self)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spie0(&mut self) -> SPIE0_W<7> {
        SPIE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr0](index.html) module"]
pub struct SPCR0_SPEC;
impl crate::RegisterSpec for SPCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spcr0::R](R) reader structure"]
impl crate::Readable for SPCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr0::W](W) writer structure"]
impl crate::Writable for SPCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR0 to value 0"]
impl crate::Resettable for SPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
