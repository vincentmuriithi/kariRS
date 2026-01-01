#[doc = "Register `SPSR` reader"]
pub struct R(crate::R<SPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPSR` writer"]
pub struct W(crate::W<SPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPSR_SPEC>;
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
impl From<crate::W<SPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2X` reader - Double SPI Speed Bit"]
pub type SPI2X_R = crate::BitReader<bool>;
#[doc = "Field `SPI2X` writer - Double SPI Speed Bit"]
pub type SPI2X_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SPSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `WCOL` reader - Write Collision Flag"]
pub type WCOL_R = crate::BitReader<bool>;
#[doc = "Field `SPIF` reader - SPI Interrupt Flag"]
pub type SPIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    pub fn spi2x(&self) -> SPI2X_R {
        SPI2X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x1f)
    }
    #[doc = "Bit 6 - Write Collision Flag"]
    #[inline(always)]
    pub fn wcol(&self) -> WCOL_R {
        WCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Flag"]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2x(&mut self) -> SPI2X_W<0> {
        SPI2X_W::new(self)
    }
    #[doc = "Bits 1:5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<1> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spsr](index.html) module"]
pub struct SPSR_SPEC;
impl crate::RegisterSpec for SPSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spsr::R](R) reader structure"]
impl crate::Readable for SPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spsr::W](W) writer structure"]
impl crate::Writable for SPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSR to value 0"]
impl crate::Resettable for SPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
