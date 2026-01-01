#[doc = "Register `INTCTRLB` reader"]
pub struct R(crate::R<INTCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRLB` writer"]
pub struct W(crate::W<INTCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRLB_SPEC>;
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
impl From<crate::W<INTCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETUP` reader - SETUP Transaction Complete Interrupt Enable"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP Transaction Complete Interrupt Enable"]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLB_SPEC, bool, O>;
#[doc = "Field `GNDONE` reader - GNAK Operation Done Interrupt Enable"]
pub type GNDONE_R = crate::BitReader<bool>;
#[doc = "Field `GNDONE` writer - GNAK Operation Done Interrupt Enable"]
pub type GNDONE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLB_SPEC, bool, O>;
#[doc = "Field `TRNCOMPL` reader - Transaction Complete Interrupt Enable"]
pub type TRNCOMPL_R = crate::BitReader<bool>;
#[doc = "Field `TRNCOMPL` writer - Transaction Complete Interrupt Enable"]
pub type TRNCOMPL_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SETUP Transaction Complete Interrupt Enable"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GNAK Operation Done Interrupt Enable"]
    #[inline(always)]
    pub fn gndone(&self) -> GNDONE_R {
        GNDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Transaction Complete Interrupt Enable"]
    #[inline(always)]
    pub fn trncompl(&self) -> TRNCOMPL_R {
        TRNCOMPL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SETUP Transaction Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<0> {
        SETUP_W::new(self)
    }
    #[doc = "Bit 1 - GNAK Operation Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gndone(&mut self) -> GNDONE_W<1> {
        GNDONE_W::new(self)
    }
    #[doc = "Bit 5 - Transaction Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trncompl(&mut self) -> TRNCOMPL_W<5> {
        TRNCOMPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrlb](index.html) module"]
pub struct INTCTRLB_SPEC;
impl crate::RegisterSpec for INTCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrlb::R](R) reader structure"]
impl crate::Readable for INTCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrlb::W](W) writer structure"]
impl crate::Writable for INTCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRLB to value 0"]
impl crate::Resettable for INTCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
