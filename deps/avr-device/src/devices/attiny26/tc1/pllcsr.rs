#[doc = "Register `PLLCSR` reader"]
pub struct R(crate::R<PLLCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCSR` writer"]
pub struct W(crate::W<PLLCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCSR_SPEC>;
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
impl From<crate::W<PLLCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLOCK` reader - PLL Lock Detector"]
pub type PLOCK_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK` writer - PLL Lock Detector"]
pub type PLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCSR_SPEC, bool, O>;
#[doc = "Field `PLLE` reader - PLL Enable"]
pub type PLLE_R = crate::BitReader<bool>;
#[doc = "Field `PLLE` writer - PLL Enable"]
pub type PLLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCSR_SPEC, bool, O>;
#[doc = "Field `PCKE` reader - PCK Enable"]
pub type PCKE_R = crate::BitReader<bool>;
#[doc = "Field `PCKE` writer - PCK Enable"]
pub type PCKE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL Lock Detector"]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Enable"]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCK Enable"]
    #[inline(always)]
    pub fn pcke(&self) -> PCKE_R {
        PCKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock Detector"]
    #[inline(always)]
    #[must_use]
    pub fn plock(&mut self) -> PLOCK_W<0> {
        PLOCK_W::new(self)
    }
    #[doc = "Bit 1 - PLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn plle(&mut self) -> PLLE_W<1> {
        PLLE_W::new(self)
    }
    #[doc = "Bit 2 - PCK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcke(&mut self) -> PCKE_W<2> {
        PCKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcsr](index.html) module"]
pub struct PLLCSR_SPEC;
impl crate::RegisterSpec for PLLCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pllcsr::R](R) reader structure"]
impl crate::Readable for PLLCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcsr::W](W) writer structure"]
impl crate::Writable for PLLCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCSR to value 0"]
impl crate::Resettable for PLLCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
