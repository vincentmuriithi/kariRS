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
#[doc = "Field `PLOCK` reader - PLL Lock Status Bit"]
pub type PLOCK_R = crate::BitReader<bool>;
#[doc = "Field `PLLE` reader - PLL Enable Bit"]
pub type PLLE_R = crate::BitReader<bool>;
#[doc = "Field `PLLE` writer - PLL Enable Bit"]
pub type PLLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCSR_SPEC, bool, O>;
#[doc = "Field `PINDIV` reader - PLL prescaler Bit 2"]
pub type PINDIV_R = crate::BitReader<bool>;
#[doc = "Field `PINDIV` writer - PLL prescaler Bit 2"]
pub type PINDIV_W<'a, const O: u8> = crate::BitWriter<'a, u8, PLLCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL Lock Status Bit"]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Enable Bit"]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL prescaler Bit 2"]
    #[inline(always)]
    pub fn pindiv(&self) -> PINDIV_R {
        PINDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PLL Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn plle(&mut self) -> PLLE_W<1> {
        PLLE_W::new(self)
    }
    #[doc = "Bit 4 - PLL prescaler Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pindiv(&mut self) -> PINDIV_W<4> {
        PINDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcsr](index.html) module"]
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
