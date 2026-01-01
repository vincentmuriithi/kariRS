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
#[doc = "Field `WGM00` reader - Waveform Generation Mode"]
pub type WGM00_R = crate::BitReader<bool>;
#[doc = "Field `WGM00` writer - Waveform Generation Mode"]
pub type WGM00_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0A_SPEC, bool, O>;
#[doc = "Field `ACIC0` reader - Analog Comparator Input Capture Enable"]
pub type ACIC0_R = crate::BitReader<bool>;
#[doc = "Field `ACIC0` writer - Analog Comparator Input Capture Enable"]
pub type ACIC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0A_SPEC, bool, O>;
#[doc = "Field `ICES0` reader - Input Capture Edge Select"]
pub type ICES0_R = crate::BitReader<bool>;
#[doc = "Field `ICES0` writer - Input Capture Edge Select"]
pub type ICES0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0A_SPEC, bool, O>;
#[doc = "Field `ICNC0` reader - Input Capture Noice Canceler"]
pub type ICNC0_R = crate::BitReader<bool>;
#[doc = "Field `ICNC0` writer - Input Capture Noice Canceler"]
pub type ICNC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0A_SPEC, bool, O>;
#[doc = "Field `ICEN0` reader - Input Capture Mode Enable"]
pub type ICEN0_R = crate::BitReader<bool>;
#[doc = "Field `ICEN0` writer - Input Capture Mode Enable"]
pub type ICEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0A_SPEC, bool, O>;
#[doc = "Field `TCW0` reader - Timer/Counter 0 Width"]
pub type TCW0_R = crate::BitReader<bool>;
#[doc = "Field `TCW0` writer - Timer/Counter 0 Width"]
pub type TCW0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0A_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm00(&self) -> WGM00_R {
        WGM00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    pub fn acic0(&self) -> ACIC0_R {
        ACIC0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn ices0(&self) -> ICES0_R {
        ICES0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Capture Noice Canceler"]
    #[inline(always)]
    pub fn icnc0(&self) -> ICNC0_R {
        ICNC0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture Mode Enable"]
    #[inline(always)]
    pub fn icen0(&self) -> ICEN0_R {
        ICEN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter 0 Width"]
    #[inline(always)]
    pub fn tcw0(&self) -> TCW0_R {
        TCW0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm00(&mut self) -> WGM00_W<0> {
        WGM00_W::new(self)
    }
    #[doc = "Bit 3 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acic0(&mut self) -> ACIC0_W<3> {
        ACIC0_W::new(self)
    }
    #[doc = "Bit 4 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices0(&mut self) -> ICES0_W<4> {
        ICES0_W::new(self)
    }
    #[doc = "Bit 5 - Input Capture Noice Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc0(&mut self) -> ICNC0_W<5> {
        ICNC0_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen0(&mut self) -> ICEN0_W<6> {
        ICEN0_W::new(self)
    }
    #[doc = "Bit 7 - Timer/Counter 0 Width"]
    #[inline(always)]
    #[must_use]
    pub fn tcw0(&mut self) -> TCW0_W<7> {
        TCW0_W::new(self)
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
