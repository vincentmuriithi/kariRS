#[doc = "Register `SPCR` reader"]
pub struct R(crate::R<SPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR` writer"]
pub struct W(crate::W<SPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR_SPEC>;
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
impl From<crate::W<SPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPR` reader - SPI Clock Rate Selects"]
pub type SPR_R = crate::FieldReader<u8, SPR_A>;
#[doc = "SPI Clock Rate Selects\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPR_A {
    #[doc = "0: Fosc/4 if SPI2X == 0 else Fosc/2"]
    FOSC_4_2 = 0,
    #[doc = "1: Fosc/16 if SPI2X == 0 else Fosc/8"]
    FOSC_16_8 = 1,
    #[doc = "2: Fosc/64 if SPI2X == 0 else Fosc/32"]
    FOSC_64_32 = 2,
    #[doc = "3: Fosc/128 if SPI2X == 0 else Fosc/64"]
    FOSC_128_64 = 3,
}
impl From<SPR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPR_A) -> Self {
        variant as _
    }
}
impl SPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPR_A {
        match self.bits {
            0 => SPR_A::FOSC_4_2,
            1 => SPR_A::FOSC_16_8,
            2 => SPR_A::FOSC_64_32,
            3 => SPR_A::FOSC_128_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FOSC_4_2`"]
    #[inline(always)]
    pub fn is_fosc_4_2(&self) -> bool {
        *self == SPR_A::FOSC_4_2
    }
    #[doc = "Checks if the value of the field is `FOSC_16_8`"]
    #[inline(always)]
    pub fn is_fosc_16_8(&self) -> bool {
        *self == SPR_A::FOSC_16_8
    }
    #[doc = "Checks if the value of the field is `FOSC_64_32`"]
    #[inline(always)]
    pub fn is_fosc_64_32(&self) -> bool {
        *self == SPR_A::FOSC_64_32
    }
    #[doc = "Checks if the value of the field is `FOSC_128_64`"]
    #[inline(always)]
    pub fn is_fosc_128_64(&self) -> bool {
        *self == SPR_A::FOSC_128_64
    }
}
#[doc = "Field `SPR` writer - SPI Clock Rate Selects"]
pub type SPR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SPCR_SPEC, u8, SPR_A, 2, O>;
impl<'a, const O: u8> SPR_W<'a, O> {
    #[doc = "Fosc/4 if SPI2X == 0 else Fosc/2"]
    #[inline(always)]
    pub fn fosc_4_2(self) -> &'a mut W {
        self.variant(SPR_A::FOSC_4_2)
    }
    #[doc = "Fosc/16 if SPI2X == 0 else Fosc/8"]
    #[inline(always)]
    pub fn fosc_16_8(self) -> &'a mut W {
        self.variant(SPR_A::FOSC_16_8)
    }
    #[doc = "Fosc/64 if SPI2X == 0 else Fosc/32"]
    #[inline(always)]
    pub fn fosc_64_32(self) -> &'a mut W {
        self.variant(SPR_A::FOSC_64_32)
    }
    #[doc = "Fosc/128 if SPI2X == 0 else Fosc/64"]
    #[inline(always)]
    pub fn fosc_128_64(self) -> &'a mut W {
        self.variant(SPR_A::FOSC_128_64)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, bool, O>;
#[doc = "Field `MSTR` reader - Master/Slave Select"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - Master/Slave Select"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, bool, O>;
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader<bool>;
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, bool, O>;
#[doc = "Field `SPE` reader - SPI Enable"]
pub type SPE_R = crate::BitReader<bool>;
#[doc = "Field `SPE` writer - SPI Enable"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, bool, O>;
#[doc = "Field `SPIE` reader - SPI Interrupt Enable"]
pub type SPIE_R = crate::BitReader<bool>;
#[doc = "Field `SPIE` writer - SPI Interrupt Enable"]
pub type SPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SPI Clock Rate Selects"]
    #[inline(always)]
    pub fn spr(&self) -> SPR_R {
        SPR_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Clock Rate Selects"]
    #[inline(always)]
    #[must_use]
    pub fn spr(&mut self) -> SPR_W<0> {
        SPR_W::new(self)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<2> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<3> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<4> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<5> {
        DORD_W::new(self)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<6> {
        SPE_W::new(self)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SPIE_W<7> {
        SPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr](index.html) module"]
pub struct SPCR_SPEC;
impl crate::RegisterSpec for SPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spcr::R](R) reader structure"]
impl crate::Readable for SPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr::W](W) writer structure"]
impl crate::Writable for SPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR to value 0"]
impl crate::Resettable for SPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
