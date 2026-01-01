#[doc = "Register `INTCTRL` reader"]
pub struct R(crate::R<INTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL` writer"]
pub struct W(crate::W<INTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRL_SPEC>;
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
impl From<crate::W<INTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESRDY` reader - Result Ready Interrupt Enable"]
pub type RESRDY_R = crate::BitReader<bool>;
#[doc = "Field `RESRDY` writer - Result Ready Interrupt Enable"]
pub type RESRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `SAMPRDY` reader - Sample Ready Interrupt Enable"]
pub type SAMPRDY_R = crate::BitReader<bool>;
#[doc = "Field `SAMPRDY` writer - Sample Ready Interrupt Enable"]
pub type SAMPRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `WCMP` reader - Window Comparator Interrupt Enable"]
pub type WCMP_R = crate::BitReader<bool>;
#[doc = "Field `WCMP` writer - Window Comparator Interrupt Enable"]
pub type WCMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `RESOVR` reader - Result Overwrite Interrupt Enable"]
pub type RESOVR_R = crate::BitReader<bool>;
#[doc = "Field `RESOVR` writer - Result Overwrite Interrupt Enable"]
pub type RESOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `SAMPOVR` reader - Sample Overwrite Interrupt Enable"]
pub type SAMPOVR_R = crate::BitReader<bool>;
#[doc = "Field `SAMPOVR` writer - Sample Overwrite Interrupt Enable"]
pub type SAMPOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `TRIGOVR` reader - Trigger Overrun Interrupt Enable"]
pub type TRIGOVR_R = crate::BitReader<bool>;
#[doc = "Field `TRIGOVR` writer - Trigger Overrun Interrupt Enable"]
pub type TRIGOVR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Result Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy(&self) -> RESRDY_R {
        RESRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sample Ready Interrupt Enable"]
    #[inline(always)]
    pub fn samprdy(&self) -> SAMPRDY_R {
        SAMPRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Comparator Interrupt Enable"]
    #[inline(always)]
    pub fn wcmp(&self) -> WCMP_R {
        WCMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn resovr(&self) -> RESOVR_R {
        RESOVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn sampovr(&self) -> SAMPOVR_R {
        SAMPOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn trigovr(&self) -> TRIGOVR_R {
        TRIGOVR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy(&mut self) -> RESRDY_W<0> {
        RESRDY_W::new(self)
    }
    #[doc = "Bit 1 - Sample Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn samprdy(&mut self) -> SAMPRDY_W<1> {
        SAMPRDY_W::new(self)
    }
    #[doc = "Bit 2 - Window Comparator Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wcmp(&mut self) -> WCMP_W<2> {
        WCMP_W::new(self)
    }
    #[doc = "Bit 3 - Result Overwrite Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resovr(&mut self) -> RESOVR_W<3> {
        RESOVR_W::new(self)
    }
    #[doc = "Bit 4 - Sample Overwrite Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sampovr(&mut self) -> SAMPOVR_W<4> {
        SAMPOVR_W::new(self)
    }
    #[doc = "Bit 5 - Trigger Overrun Interrupt Enable"]
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
#[doc = "Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl](index.html) module"]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrl::R](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrl::W](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
