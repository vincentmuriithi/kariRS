#[doc = "Register `TCCR0A` reader"]
pub struct R(crate::R<TCCR0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR0A` writer"]
pub struct W(crate::W<TCCR0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR0A_SPEC>;
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
impl From<crate::W<TCCR0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGM0` reader - Waveform Generation Mode bits"]
pub type WGM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM0` writer - Waveform Generation Mode bits"]
pub type WGM0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM0B` reader - Compare Match Output B Mode bits"]
pub type COM0B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM0B` writer - Compare Match Output B Mode bits"]
pub type COM0B_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0A_SPEC, u8, u8, 2, O>;
#[doc = "Field `COM0A` reader - Compare Match Output A Mode bits"]
pub type COM0A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM0A` writer - Compare Match Output A Mode bits"]
pub type COM0A_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0A_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode bits"]
    #[inline(always)]
    pub fn wgm0(&self) -> WGM0_R {
        WGM0_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode bits"]
    #[inline(always)]
    pub fn com0b(&self) -> COM0B_R {
        COM0B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Match Output A Mode bits"]
    #[inline(always)]
    pub fn com0a(&self) -> COM0A_R {
        COM0A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm0(&mut self) -> WGM0_W<0> {
        WGM0_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com0b(&mut self) -> COM0B_W<4> {
        COM0B_W::new(self)
    }
    #[doc = "Bits 6:7 - Compare Match Output A Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com0a(&mut self) -> COM0A_W<6> {
        COM0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr0a](index.html) module"]
pub struct TCCR0A_SPEC;
impl crate::RegisterSpec for TCCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr0a::R](R) reader structure"]
impl crate::Readable for TCCR0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr0a::W](W) writer structure"]
impl crate::Writable for TCCR0A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0A to value 0"]
impl crate::Resettable for TCCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
