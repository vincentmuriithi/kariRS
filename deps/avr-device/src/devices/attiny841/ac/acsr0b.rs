#[doc = "Register `ACSR0B` reader"]
pub struct R(crate::R<ACSR0B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR0B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSR0B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSR0B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR0B` writer"]
pub struct W(crate::W<ACSR0B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR0B_SPEC>;
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
impl From<crate::W<ACSR0B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSR0B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACPMUX` reader - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACPMUX` writer - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ACSR0B_SPEC, u8, u8, 2, O>;
#[doc = "Field `ACNMUX` reader - Analog Comparator 0 Negative Input Multiplexer"]
pub type ACNMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACNMUX` writer - Analog Comparator 0 Negative Input Multiplexer"]
pub type ACNMUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ACSR0B_SPEC, u8, u8, 2, O>;
#[doc = "Field `ACOE0` reader - Analog Comparator 0 Output Pin Enable"]
pub type ACOE0_R = crate::BitReader<bool>;
#[doc = "Field `ACOE0` writer - Analog Comparator 0 Output Pin Enable"]
pub type ACOE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR0B_SPEC, bool, O>;
#[doc = "Field `HLEV0` reader - Analog Comparator 0 Hysteresis Level"]
pub type HLEV0_R = crate::BitReader<bool>;
#[doc = "Field `HLEV0` writer - Analog Comparator 0 Hysteresis Level"]
pub type HLEV0_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR0B_SPEC, bool, O>;
#[doc = "Field `HSEL0` reader - Analog Comparator 0 Hysteresis Select"]
pub type HSEL0_R = crate::BitReader<bool>;
#[doc = "Field `HSEL0` writer - Analog Comparator 0 Hysteresis Select"]
pub type HSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR0B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    pub fn acpmux(&self) -> ACPMUX_R {
        ACPMUX_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Analog Comparator 0 Negative Input Multiplexer"]
    #[inline(always)]
    pub fn acnmux(&self) -> ACNMUX_R {
        ACNMUX_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Output Pin Enable"]
    #[inline(always)]
    pub fn acoe0(&self) -> ACOE0_R {
        ACOE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 0 Hysteresis Level"]
    #[inline(always)]
    pub fn hlev0(&self) -> HLEV0_R {
        HLEV0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Hysteresis Select"]
    #[inline(always)]
    pub fn hsel0(&self) -> HSEL0_R {
        HSEL0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn acpmux(&mut self) -> ACPMUX_W<0> {
        ACPMUX_W::new(self)
    }
    #[doc = "Bits 2:3 - Analog Comparator 0 Negative Input Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn acnmux(&mut self) -> ACNMUX_W<2> {
        ACNMUX_W::new(self)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acoe0(&mut self) -> ACOE0_W<4> {
        ACOE0_W::new(self)
    }
    #[doc = "Bit 6 - Analog Comparator 0 Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hlev0(&mut self) -> HLEV0_W<6> {
        HLEV0_W::new(self)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Hysteresis Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsel0(&mut self) -> HSEL0_W<7> {
        HSEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator 0 Control And Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr0b](index.html) module"]
pub struct ACSR0B_SPEC;
impl crate::RegisterSpec for ACSR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acsr0b::R](R) reader structure"]
impl crate::Readable for ACSR0B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr0b::W](W) writer structure"]
impl crate::Writable for ACSR0B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR0B to value 0"]
impl crate::Resettable for ACSR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
