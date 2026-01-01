#[doc = "Register `TCCR2A` reader"]
pub struct R(crate::R<TCCR2A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR2A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR2A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR2A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR2A` writer"]
pub struct W(crate::W<TCCR2A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR2A_SPEC>;
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
impl From<crate::W<TCCR2A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR2A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM2` reader - Waveform Genration Mode"]
pub type WGM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM2` writer - Waveform Genration Mode"]
pub type WGM2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM2B` reader - Compare Output Mode bits"]
pub type COM2B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM2B` writer - Compare Output Mode bits"]
pub type COM2B_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM2A` reader - Compare Output Mode bits"]
pub type COM2A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM2A` writer - Compare Output Mode bits"]
pub type COM2A_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2A_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    pub fn wgm2(&self) -> WGM2_R {
        WGM2_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com2b(&self) -> COM2B_R {
        COM2B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com2a(&self) -> COM2A_R {
        COM2A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm2(&mut self) -> WGM2_W<0> {
        WGM2_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com2b(&mut self) -> COM2B_W<4> {
        COM2B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com2a(&mut self) -> COM2A_W<6> {
        COM2A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter2 Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr2a](index.html) module"]
pub struct TCCR2A_SPEC;
impl crate::RegisterSpec for TCCR2A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr2a::R](R) reader structure"]
impl crate::Readable for TCCR2A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr2a::W](W) writer structure"]
impl crate::Writable for TCCR2A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2A to value 0"]
impl crate::Resettable for TCCR2A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
