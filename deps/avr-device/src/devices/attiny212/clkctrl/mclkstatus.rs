#[doc = "Register `MCLKSTATUS` reader"]
pub struct R(crate::R<MCLKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKSTATUS` writer"]
pub struct W(crate::W<MCLKSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKSTATUS_SPEC>;
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
impl From<crate::W<MCLKSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOSC` reader - System Oscillator changing"]
pub type SOSC_R = crate::BitReader<bool>;
#[doc = "Field `SOSC` writer - System Oscillator changing"]
pub type SOSC_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKSTATUS_SPEC, bool, O>;
#[doc = "Field `OSC20MS` reader - 20MHz oscillator status"]
pub type OSC20MS_R = crate::BitReader<bool>;
#[doc = "Field `OSC20MS` writer - 20MHz oscillator status"]
pub type OSC20MS_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKSTATUS_SPEC, bool, O>;
#[doc = "Field `OSC32KS` reader - 32KHz oscillator status"]
pub type OSC32KS_R = crate::BitReader<bool>;
#[doc = "Field `OSC32KS` writer - 32KHz oscillator status"]
pub type OSC32KS_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKSTATUS_SPEC, bool, O>;
#[doc = "Field `XOSC32KS` reader - 32.768 kHz Crystal Oscillator status"]
pub type XOSC32KS_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KS` writer - 32.768 kHz Crystal Oscillator status"]
pub type XOSC32KS_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKSTATUS_SPEC, bool, O>;
#[doc = "Field `EXTS` reader - External Clock status"]
pub type EXTS_R = crate::BitReader<bool>;
#[doc = "Field `EXTS` writer - External Clock status"]
pub type EXTS_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - System Oscillator changing"]
    #[inline(always)]
    pub fn sosc(&self) -> SOSC_R {
        SOSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 20MHz oscillator status"]
    #[inline(always)]
    pub fn osc20ms(&self) -> OSC20MS_R {
        OSC20MS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 32KHz oscillator status"]
    #[inline(always)]
    pub fn osc32ks(&self) -> OSC32KS_R {
        OSC32KS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 32.768 kHz Crystal Oscillator status"]
    #[inline(always)]
    pub fn xosc32ks(&self) -> XOSC32KS_R {
        XOSC32KS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Clock status"]
    #[inline(always)]
    pub fn exts(&self) -> EXTS_R {
        EXTS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Oscillator changing"]
    #[inline(always)]
    #[must_use]
    pub fn sosc(&mut self) -> SOSC_W<0> {
        SOSC_W::new(self)
    }
    #[doc = "Bit 4 - 20MHz oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn osc20ms(&mut self) -> OSC20MS_W<4> {
        OSC20MS_W::new(self)
    }
    #[doc = "Bit 5 - 32KHz oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn osc32ks(&mut self) -> OSC32KS_W<5> {
        OSC32KS_W::new(self)
    }
    #[doc = "Bit 6 - 32.768 kHz Crystal Oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32ks(&mut self) -> XOSC32KS_W<6> {
        XOSC32KS_W::new(self)
    }
    #[doc = "Bit 7 - External Clock status"]
    #[inline(always)]
    #[must_use]
    pub fn exts(&mut self) -> EXTS_W<7> {
        EXTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkstatus](index.html) module"]
pub struct MCLKSTATUS_SPEC;
impl crate::RegisterSpec for MCLKSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclkstatus::R](R) reader structure"]
impl crate::Readable for MCLKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkstatus::W](W) writer structure"]
impl crate::Writable for MCLKSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKSTATUS to value 0"]
impl crate::Resettable for MCLKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
