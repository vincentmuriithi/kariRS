#[doc = "Register `TCCR1A` reader"]
pub struct R(crate::R<TCCR1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1A` writer"]
pub struct W(crate::W<TCCR1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1A_SPEC>;
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
impl From<crate::W<TCCR1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM1` reader - Waveform Generation Mode"]
pub type WGM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM1` writer - Waveform Generation Mode"]
pub type WGM1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM1C` reader - Compare Output Mode 1C, bits"]
pub type COM1C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM1C` writer - Compare Output Mode 1C, bits"]
pub type COM1C_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM1B` reader - Compare Output Mode 1B, bits"]
pub type COM1B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM1B` writer - Compare Output Mode 1B, bits"]
pub type COM1B_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM1A` reader - Compare Output Mode 1A, bits"]
pub type COM1A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM1A` writer - Compare Output Mode 1A, bits"]
pub type COM1A_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1A_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm1(&self) -> WGM1_R {
        WGM1_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 1C, bits"]
    #[inline(always)]
    pub fn com1c(&self) -> COM1C_R {
        COM1C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 1B, bits"]
    #[inline(always)]
    pub fn com1b(&self) -> COM1B_R {
        COM1B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    pub fn com1a(&self) -> COM1A_R {
        COM1A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm1(&mut self) -> WGM1_W<0> {
        WGM1_W::new(self)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 1C, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com1c(&mut self) -> COM1C_W<2> {
        COM1C_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 1B, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com1b(&mut self) -> COM1B_W<4> {
        COM1B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com1a(&mut self) -> COM1A_W<6> {
        COM1A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter1 Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1a](index.html) module"]
pub struct TCCR1A_SPEC;
impl crate::RegisterSpec for TCCR1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1a::R](R) reader structure"]
impl crate::Readable for TCCR1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1a::W](W) writer structure"]
impl crate::Writable for TCCR1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1A to value 0"]
impl crate::Resettable for TCCR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
