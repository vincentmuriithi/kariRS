#[doc = "Register `TIMSK3` reader"]
pub struct R(crate::R<TIMSK3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK3` writer"]
pub struct W(crate::W<TIMSK3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK3_SPEC>;
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
impl From<crate::W<TIMSK3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE3` reader - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_R = crate::BitReader<bool>;
#[doc = "Field `TOIE3` writer - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK3_SPEC, bool, O>;
#[doc = "Field `OCIE3A` reader - Timer/Counter3 Output Compare Match A Interrupt Enable"]
pub type OCIE3A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE3A` writer - Timer/Counter3 Output Compare Match A Interrupt Enable"]
pub type OCIE3A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK3_SPEC, bool, O>;
#[doc = "Field `OCIE3B` reader - Timer/Counter3 Output Compare Match B Interrupt Enable"]
pub type OCIE3B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE3B` writer - Timer/Counter3 Output Compare Match B Interrupt Enable"]
pub type OCIE3B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK3_SPEC, bool, O>;
#[doc = "Field `ICIE3` reader - Timer/Counter3 Input Capture Interrupt Enable"]
pub type ICIE3_R = crate::BitReader<bool>;
#[doc = "Field `ICIE3` writer - Timer/Counter3 Input Capture Interrupt Enable"]
pub type ICIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie3(&self) -> TOIE3_R {
        TOIE3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter3 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3a(&self) -> OCIE3A_R {
        OCIE3A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter3 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3b(&self) -> OCIE3B_R {
        OCIE3B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie3(&self) -> ICIE3_R {
        ICIE3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie3(&mut self) -> TOIE3_W<0> {
        TOIE3_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter3 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3a(&mut self) -> OCIE3A_W<1> {
        OCIE3A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter3 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3b(&mut self) -> OCIE3B_W<2> {
        OCIE3B_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie3(&mut self) -> ICIE3_W<5> {
        ICIE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk3](index.html) module"]
pub struct TIMSK3_SPEC;
impl crate::RegisterSpec for TIMSK3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk3::R](R) reader structure"]
impl crate::Readable for TIMSK3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk3::W](W) writer structure"]
impl crate::Writable for TIMSK3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK3 to value 0"]
impl crate::Resettable for TIMSK3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
