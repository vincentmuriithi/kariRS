#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVL0RR` reader - Round-robin Scheduling Enable"]
pub type LVL0RR_R = crate::BitReader<bool>;
#[doc = "Field `LVL0RR` writer - Round-robin Scheduling Enable"]
pub type LVL0RR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `CVT` reader - Compact Vector Table"]
pub type CVT_R = crate::BitReader<bool>;
#[doc = "Field `CVT` writer - Compact Vector Table"]
pub type CVT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader<bool>;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Round-robin Scheduling Enable"]
    #[inline(always)]
    pub fn lvl0rr(&self) -> LVL0RR_R {
        LVL0RR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Compact Vector Table"]
    #[inline(always)]
    pub fn cvt(&self) -> CVT_R {
        CVT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Vector Select"]
    #[inline(always)]
    pub fn ivsel(&self) -> IVSEL_R {
        IVSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Round-robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvl0rr(&mut self) -> LVL0RR_W<0> {
        LVL0RR_W::new(self)
    }
    #[doc = "Bit 5 - Compact Vector Table"]
    #[inline(always)]
    #[must_use]
    pub fn cvt(&mut self) -> CVT_W<5> {
        CVT_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<6> {
        IVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
