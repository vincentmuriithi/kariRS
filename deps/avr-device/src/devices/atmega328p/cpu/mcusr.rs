#[doc = "Register `MCUSR` reader"]
pub struct R(crate::R<MCUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUSR` writer"]
pub struct W(crate::W<MCUSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUSR_SPEC>;
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
impl From<crate::W<MCUSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORF` reader - Power-on reset flag"]
pub type PORF_R = crate::BitReader<bool>;
#[doc = "Field `PORF` writer - Power-on reset flag"]
pub type PORF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUSR_SPEC, bool, O>;
#[doc = "Field `EXTRF` reader - External Reset Flag"]
pub type EXTRF_R = crate::BitReader<bool>;
#[doc = "Field `EXTRF` writer - External Reset Flag"]
pub type EXTRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUSR_SPEC, bool, O>;
#[doc = "Field `BORF` reader - Brown-out Reset Flag"]
pub type BORF_R = crate::BitReader<bool>;
#[doc = "Field `BORF` writer - Brown-out Reset Flag"]
pub type BORF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUSR_SPEC, bool, O>;
#[doc = "Field `WDRF` reader - Watchdog Reset Flag"]
pub type WDRF_R = crate::BitReader<bool>;
#[doc = "Field `WDRF` writer - Watchdog Reset Flag"]
pub type WDRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power-on reset flag"]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reset Flag"]
    #[inline(always)]
    pub fn extrf(&self) -> EXTRF_R {
        EXTRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown-out Reset Flag"]
    #[inline(always)]
    pub fn borf(&self) -> BORF_R {
        BORF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Reset Flag"]
    #[inline(always)]
    pub fn wdrf(&self) -> WDRF_R {
        WDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-on reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porf(&mut self) -> PORF_W<0> {
        PORF_W::new(self)
    }
    #[doc = "Bit 1 - External Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extrf(&mut self) -> EXTRF_W<1> {
        EXTRF_W::new(self)
    }
    #[doc = "Bit 2 - Brown-out Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn borf(&mut self) -> BORF_W<2> {
        BORF_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdrf(&mut self) -> WDRF_W<3> {
        WDRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcusr](index.html) module"]
pub struct MCUSR_SPEC;
impl crate::RegisterSpec for MCUSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcusr::R](R) reader structure"]
impl crate::Readable for MCUSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcusr::W](W) writer structure"]
impl crate::Writable for MCUSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUSR to value 0"]
impl crate::Resettable for MCUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
