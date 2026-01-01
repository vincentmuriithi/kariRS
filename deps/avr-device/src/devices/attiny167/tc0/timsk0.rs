#[doc = "Register `TIMSK0` reader"]
pub struct R(crate::R<TIMSK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK0` writer"]
pub struct W(crate::W<TIMSK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK0_SPEC>;
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
impl From<crate::W<TIMSK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE0` reader - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_R = crate::BitReader<bool>;
#[doc = "Field `TOIE0` writer - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK0_SPEC, bool, O>;
#[doc = "Field `OCIE0A` reader - Timer/Counter0 Output Compare Match A Interrupt Enable"]
pub type OCIE0A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE0A` writer - Timer/Counter0 Output Compare Match A Interrupt Enable"]
pub type OCIE0A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie0(&self) -> TOIE0_R {
        TOIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie0a(&self) -> OCIE0A_R {
        OCIE0A_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie0(&mut self) -> TOIE0_W<0> {
        TOIE0_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie0a(&mut self) -> OCIE0A_W<1> {
        OCIE0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter0 Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk0](index.html) module"]
pub struct TIMSK0_SPEC;
impl crate::RegisterSpec for TIMSK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk0::R](R) reader structure"]
impl crate::Readable for TIMSK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk0::W](W) writer structure"]
impl crate::Writable for TIMSK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK0 to value 0"]
impl crate::Resettable for TIMSK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
