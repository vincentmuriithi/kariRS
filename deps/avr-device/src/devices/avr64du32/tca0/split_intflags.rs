#[doc = "Register `INTFLAGS` reader"]
pub struct R(crate::R<SPLIT_INTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLIT_INTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLIT_INTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLIT_INTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGS` writer"]
pub struct W(crate::W<SPLIT_INTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLIT_INTFLAGS_SPEC>;
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
impl From<crate::W<SPLIT_INTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLIT_INTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUNF` reader - Low Underflow Interrupt Flag"]
pub type LUNF_R = crate::BitReader<bool>;
#[doc = "Field `LUNF` writer - Low Underflow Interrupt Flag"]
pub type LUNF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_INTFLAGS_SPEC, bool, O>;
#[doc = "Field `HUNF` reader - High Underflow Interrupt Flag"]
pub type HUNF_R = crate::BitReader<bool>;
#[doc = "Field `HUNF` writer - High Underflow Interrupt Flag"]
pub type HUNF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_INTFLAGS_SPEC, bool, O>;
#[doc = "Field `LCMP0` reader - Low Compare 2 Interrupt Flag"]
pub type LCMP0_R = crate::BitReader<bool>;
#[doc = "Field `LCMP0` writer - Low Compare 2 Interrupt Flag"]
pub type LCMP0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_INTFLAGS_SPEC, bool, O>;
#[doc = "Field `LCMP1` reader - Low Compare 1 Interrupt Flag"]
pub type LCMP1_R = crate::BitReader<bool>;
#[doc = "Field `LCMP1` writer - Low Compare 1 Interrupt Flag"]
pub type LCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_INTFLAGS_SPEC, bool, O>;
#[doc = "Field `LCMP2` reader - Low Compare 0 Interrupt Flag"]
pub type LCMP2_R = crate::BitReader<bool>;
#[doc = "Field `LCMP2` writer - Low Compare 0 Interrupt Flag"]
pub type LCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_INTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Low Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn lunf(&self) -> LUNF_R {
        LUNF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn hunf(&self) -> HUNF_R {
        HUNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn lcmp0(&self) -> LCMP0_R {
        LCMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn lcmp1(&self) -> LCMP1_R {
        LCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Compare 0 Interrupt Flag"]
    #[inline(always)]
    pub fn lcmp2(&self) -> LCMP2_R {
        LCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lunf(&mut self) -> LUNF_W<0> {
        LUNF_W::new(self)
    }
    #[doc = "Bit 1 - High Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hunf(&mut self) -> HUNF_W<1> {
        HUNF_W::new(self)
    }
    #[doc = "Bit 4 - Low Compare 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp0(&mut self) -> LCMP0_W<4> {
        LCMP0_W::new(self)
    }
    #[doc = "Bit 5 - Low Compare 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp1(&mut self) -> LCMP1_W<5> {
        LCMP1_W::new(self)
    }
    #[doc = "Bit 6 - Low Compare 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp2(&mut self) -> LCMP2_W<6> {
        LCMP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [split_intflags](index.html) module"]
pub struct SPLIT_INTFLAGS_SPEC;
impl crate::RegisterSpec for SPLIT_INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [split_intflags::R](R) reader structure"]
impl crate::Readable for SPLIT_INTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [split_intflags::W](W) writer structure"]
impl crate::Writable for SPLIT_INTFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::Resettable for SPLIT_INTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
