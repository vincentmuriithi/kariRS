#[doc = "Register `SPSR0` reader"]
pub struct R(crate::R<SPSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPSR0` writer"]
pub struct W(crate::W<SPSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPSR0_SPEC>;
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
impl From<crate::W<SPSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2X0` reader - Double SPI Speed Bit"]
pub type SPI2X0_R = crate::BitReader<bool>;
#[doc = "Field `SPI2X0` writer - Double SPI Speed Bit"]
pub type SPI2X0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR0_SPEC, bool, O>;
#[doc = "Field `WCOL0` reader - Write Collision Flag"]
pub type WCOL0_R = crate::BitReader<bool>;
#[doc = "Field `WCOL0` writer - Write Collision Flag"]
pub type WCOL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR0_SPEC, bool, O>;
#[doc = "Field `SPIF0` reader - SPI Interrupt Flag"]
pub type SPIF0_R = crate::BitReader<bool>;
#[doc = "Field `SPIF0` writer - SPI Interrupt Flag"]
pub type SPIF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPSR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    pub fn spi2x0(&self) -> SPI2X0_R {
        SPI2X0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - Write Collision Flag"]
    #[inline(always)]
    pub fn wcol0(&self) -> WCOL0_R {
        WCOL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Flag"]
    #[inline(always)]
    pub fn spif0(&self) -> SPIF0_R {
        SPIF0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2x0(&mut self) -> SPI2X0_W<0> {
        SPI2X0_W::new(self)
    }
    #[doc = "Bit 6 - Write Collision Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wcol0(&mut self) -> WCOL0_W<6> {
        WCOL0_W::new(self)
    }
    #[doc = "Bit 7 - SPI Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn spif0(&mut self) -> SPIF0_W<7> {
        SPIF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spsr0](index.html) module"]
pub struct SPSR0_SPEC;
impl crate::RegisterSpec for SPSR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spsr0::R](R) reader structure"]
impl crate::Readable for SPSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spsr0::W](W) writer structure"]
impl crate::Writable for SPSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSR0 to value 0"]
impl crate::Resettable for SPSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
