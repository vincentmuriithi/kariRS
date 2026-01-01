#[doc = "Register `SPIROUTEA` reader"]
pub struct R(crate::R<SPIROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIROUTEA` writer"]
pub struct W(crate::W<SPIROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIROUTEA_SPEC>;
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
impl From<crate::W<SPIROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0` reader - SPI0 Signals"]
pub type SPI0_R = crate::FieldReader<u8, SPI0_A>;
#[doc = "SPI0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI0_A {
    #[doc = "0: MOSI: PA4, MISO: PA5, SCK: PA6, SS: PA7"]
    DEFAULT = 0,
    #[doc = "4: MOSI: PD4, MISO: PD5, SCK: PD6, SS: PD7"]
    ALT4 = 4,
    #[doc = "7: Not connected to any pins, SS set to 1"]
    NONE = 7,
}
impl From<SPI0_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as _
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI0_A> {
        match self.bits {
            0 => Some(SPI0_A::DEFAULT),
            4 => Some(SPI0_A::ALT4),
            7 => Some(SPI0_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == SPI0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == SPI0_A::ALT4
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SPI0_A::NONE
    }
}
#[doc = "Field `SPI0` writer - SPI0 Signals"]
pub type SPI0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SPIROUTEA_SPEC, u8, SPI0_A, 3, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "MOSI: PA4, MISO: PA5, SCK: PA6, SS: PA7"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(SPI0_A::DEFAULT)
    }
    #[doc = "MOSI: PD4, MISO: PD5, SCK: PD6, SS: PD7"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(SPI0_A::ALT4)
    }
    #[doc = "Not connected to any pins, SS set to 1"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SPI0_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI0 Signals"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<0> {
        SPI0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiroutea](index.html) module"]
pub struct SPIROUTEA_SPEC;
impl crate::RegisterSpec for SPIROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spiroutea::R](R) reader structure"]
impl crate::Readable for SPIROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spiroutea::W](W) writer structure"]
impl crate::Writable for SPIROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIROUTEA to value 0"]
impl crate::Resettable for SPIROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
