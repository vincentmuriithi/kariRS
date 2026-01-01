#[doc = "Register `WDTCKD` reader"]
pub struct R(crate::R<WDTCKD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCKD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCKD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCKD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCKD` writer"]
pub struct W(crate::W<WDTCKD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCKD_SPEC>;
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
impl From<crate::W<WDTCKD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCKD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WCLKD` reader - Watchdog Timer Clock Dividers"]
pub type WCLKD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WCLKD` writer - Watchdog Timer Clock Dividers"]
pub type WCLKD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, WDTCKD_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDEWIE` reader - Watchdog Early Warning Interrupt Enable"]
pub type WDEWIE_R = crate::BitReader<bool>;
#[doc = "Field `WDEWIE` writer - Watchdog Early Warning Interrupt Enable"]
pub type WDEWIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCKD_SPEC, bool, O>;
#[doc = "Field `WDEWIF` reader - Watchdog Early Warning Interrupt Flag"]
pub type WDEWIF_R = crate::BitReader<bool>;
#[doc = "Field `WDEWIF` writer - Watchdog Early Warning Interrupt Flag"]
pub type WDEWIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCKD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Watchdog Timer Clock Dividers"]
    #[inline(always)]
    pub fn wclkd(&self) -> WCLKD_R {
        WCLKD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Watchdog Early Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wdewie(&self) -> WDEWIE_R {
        WDEWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Early Warning Interrupt Flag"]
    #[inline(always)]
    pub fn wdewif(&self) -> WDEWIF_R {
        WDEWIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Timer Clock Dividers"]
    #[inline(always)]
    #[must_use]
    pub fn wclkd(&mut self) -> WCLKD_W<0> {
        WCLKD_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog Early Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdewie(&mut self) -> WDEWIE_W<2> {
        WDEWIE_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog Early Warning Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdewif(&mut self) -> WDEWIF_W<3> {
        WDEWIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Clock Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtckd](index.html) module"]
pub struct WDTCKD_SPEC;
impl crate::RegisterSpec for WDTCKD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdtckd::R](R) reader structure"]
impl crate::Readable for WDTCKD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtckd::W](W) writer structure"]
impl crate::Writable for WDTCKD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCKD to value 0"]
impl crate::Resettable for WDTCKD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
