#[doc = "Register `INTCTRLA` reader"]
pub struct R(crate::R<INTCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRLA` writer"]
pub struct W(crate::W<INTCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCTRLA_SPEC>;
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
impl From<crate::W<INTCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
#[doc = "Field `UNF` reader - Underflow Interrupt Enable"]
pub type UNF_R = crate::BitReader<bool>;
#[doc = "Field `UNF` writer - Underflow Interrupt Enable"]
pub type UNF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
#[doc = "Field `STALLED` reader - STALL Interrupt Enable"]
pub type STALLED_R = crate::BitReader<bool>;
#[doc = "Field `STALLED` writer - STALL Interrupt Enable"]
pub type STALLED_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Reset Interrupt Enable"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Reset Interrupt Enable"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - Resume Interrupt Enable"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Resume Interrupt Enable"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
#[doc = "Field `SUSPEND` reader - Suspend Interrupt Enable"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` writer - Suspend Interrupt Enable"]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
#[doc = "Field `SOF` reader - Start Of Frame Interrupt Enable"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - Start Of Frame Interrupt Enable"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stalled(&self) -> STALLED_R {
        STALLED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Interrupt Enable"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<1> {
        OVF_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UNF_W<2> {
        UNF_W::new(self)
    }
    #[doc = "Bit 3 - STALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stalled(&mut self) -> STALLED_W<3> {
        STALLED_W::new(self)
    }
    #[doc = "Bit 4 - Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<4> {
        RESET_W::new(self)
    }
    #[doc = "Bit 5 - Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<5> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 6 - Suspend Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<6> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 7 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<7> {
        SOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrla](index.html) module"]
pub struct INTCTRLA_SPEC;
impl crate::RegisterSpec for INTCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrla::R](R) reader structure"]
impl crate::Readable for INTCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrla::W](W) writer structure"]
impl crate::Writable for INTCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRLA to value 0"]
impl crate::Resettable for INTCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
