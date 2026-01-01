#[doc = "Register `ACSRB` reader"]
pub struct R(crate::R<ACSRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSRB` writer"]
pub struct W(crate::W<ACSRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSRB_SPEC>;
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
impl From<crate::W<ACSRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACPMUX` reader - Analog Comparator Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACPMUX` writer - Analog Comparator Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ACSRB_SPEC, u8, u8, 2, O>;
#[doc = "Field `ACNMUX` reader - Analog Comparator Negative Input Multiplexer"]
pub type ACNMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACNMUX` writer - Analog Comparator Negative Input Multiplexer"]
pub type ACNMUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ACSRB_SPEC, u8, u8, 2, O>;
#[doc = "Field `HLEV` reader - Hysteresis Level"]
pub type HLEV_R = crate::BitReader<bool>;
#[doc = "Field `HLEV` writer - Hysteresis Level"]
pub type HLEV_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSRB_SPEC, bool, O>;
#[doc = "Field `HSEL` reader - Hysteresis Select"]
pub type HSEL_R = crate::BitReader<bool>;
#[doc = "Field `HSEL` writer - Hysteresis Select"]
pub type HSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSRB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    pub fn acpmux(&self) -> ACPMUX_R {
        ACPMUX_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Analog Comparator Negative Input Multiplexer"]
    #[inline(always)]
    pub fn acnmux(&self) -> ACNMUX_R {
        ACNMUX_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 6 - Hysteresis Level"]
    #[inline(always)]
    pub fn hlev(&self) -> HLEV_R {
        HLEV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hysteresis Select"]
    #[inline(always)]
    pub fn hsel(&self) -> HSEL_R {
        HSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn acpmux(&mut self) -> ACPMUX_W<0> {
        ACPMUX_W::new(self)
    }
    #[doc = "Bits 2:3 - Analog Comparator Negative Input Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn acnmux(&mut self) -> ACNMUX_W<2> {
        ACNMUX_W::new(self)
    }
    #[doc = "Bit 6 - Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hlev(&mut self) -> HLEV_W<6> {
        HLEV_W::new(self)
    }
    #[doc = "Bit 7 - Hysteresis Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsel(&mut self) -> HSEL_W<7> {
        HSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Control And Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsrb](index.html) module"]
pub struct ACSRB_SPEC;
impl crate::RegisterSpec for ACSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acsrb::R](R) reader structure"]
impl crate::Readable for ACSRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsrb::W](W) writer structure"]
impl crate::Writable for ACSRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSRB to value 0"]
impl crate::Resettable for ACSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
