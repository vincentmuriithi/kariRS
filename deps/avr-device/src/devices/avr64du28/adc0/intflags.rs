#[doc = "Register `INTFLAGS` reader"]
pub struct R(crate::R<INTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGS` writer"]
pub struct W(crate::W<INTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGS_SPEC>;
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
impl From<crate::W<INTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESRDY` reader - Result Ready Flag"]
pub type RESRDY_R = crate::BitReader<bool>;
#[doc = "Field `RESRDY` writer - Result Ready Flag"]
pub type RESRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `SAMPRDY` reader - Sample Ready Flag"]
pub type SAMPRDY_R = crate::BitReader<bool>;
#[doc = "Field `SAMPRDY` writer - Sample Ready Flag"]
pub type SAMPRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `WCMP` reader - Window Comparator Flag"]
pub type WCMP_R = crate::BitReader<bool>;
#[doc = "Field `WCMP` writer - Window Comparator Flag"]
pub type WCMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `RESOVR` reader - Result OverwriteFlag"]
pub type RESOVR_R = crate::BitReader<bool>;
#[doc = "Field `RESOVR` writer - Result OverwriteFlag"]
pub type RESOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `SAMPOVR` reader - Sample OverwriteFlag"]
pub type SAMPOVR_R = crate::BitReader<bool>;
#[doc = "Field `SAMPOVR` writer - Sample OverwriteFlag"]
pub type SAMPOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `TRIGOVR` reader - Trigger OverrunFlag"]
pub type TRIGOVR_R = crate::BitReader<bool>;
#[doc = "Field `TRIGOVR` writer - Trigger OverrunFlag"]
pub type TRIGOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Result Ready Flag"]
    #[inline(always)]
    pub fn resrdy(&self) -> RESRDY_R {
        RESRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sample Ready Flag"]
    #[inline(always)]
    pub fn samprdy(&self) -> SAMPRDY_R {
        SAMPRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Comparator Flag"]
    #[inline(always)]
    pub fn wcmp(&self) -> WCMP_R {
        WCMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result OverwriteFlag"]
    #[inline(always)]
    pub fn resovr(&self) -> RESOVR_R {
        RESOVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample OverwriteFlag"]
    #[inline(always)]
    pub fn sampovr(&self) -> SAMPOVR_R {
        SAMPOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger OverrunFlag"]
    #[inline(always)]
    pub fn trigovr(&self) -> TRIGOVR_R {
        TRIGOVR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Flag"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy(&mut self) -> RESRDY_W<0> {
        RESRDY_W::new(self)
    }
    #[doc = "Bit 1 - Sample Ready Flag"]
    #[inline(always)]
    #[must_use]
    pub fn samprdy(&mut self) -> SAMPRDY_W<1> {
        SAMPRDY_W::new(self)
    }
    #[doc = "Bit 2 - Window Comparator Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wcmp(&mut self) -> WCMP_W<2> {
        WCMP_W::new(self)
    }
    #[doc = "Bit 3 - Result OverwriteFlag"]
    #[inline(always)]
    #[must_use]
    pub fn resovr(&mut self) -> RESOVR_W<3> {
        RESOVR_W::new(self)
    }
    #[doc = "Bit 4 - Sample OverwriteFlag"]
    #[inline(always)]
    #[must_use]
    pub fn sampovr(&mut self) -> SAMPOVR_W<4> {
        SAMPOVR_W::new(self)
    }
    #[doc = "Bit 5 - Trigger OverrunFlag"]
    #[inline(always)]
    #[must_use]
    pub fn trigovr(&mut self) -> TRIGOVR_W<5> {
        TRIGOVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflags](index.html) module"]
pub struct INTFLAGS_SPEC;
impl crate::RegisterSpec for INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflags::R](R) reader structure"]
impl crate::Readable for INTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflags::W](W) writer structure"]
impl crate::Writable for INTFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::Resettable for INTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
