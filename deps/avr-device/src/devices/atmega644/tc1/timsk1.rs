#[doc = "Register `TIMSK1` reader"]
pub struct R(crate::R<TIMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK1` writer"]
pub struct W(crate::W<TIMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK1_SPEC>;
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
impl From<crate::W<TIMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE1` reader - Timer/Counter1 Overflow Interrupt Enable"]
pub type TOIE1_R = crate::BitReader<bool>;
#[doc = "Field `TOIE1` writer - Timer/Counter1 Overflow Interrupt Enable"]
pub type TOIE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK1_SPEC, bool, O>;
#[doc = "Field `OCIE1A` reader - Timer/Counter1 Output Compare A Match Interrupt Enable"]
pub type OCIE1A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE1A` writer - Timer/Counter1 Output Compare A Match Interrupt Enable"]
pub type OCIE1A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK1_SPEC, bool, O>;
#[doc = "Field `OCIE1B` reader - Timer/Counter1 Output Compare B Match Interrupt Enable"]
pub type OCIE1B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE1B` writer - Timer/Counter1 Output Compare B Match Interrupt Enable"]
pub type OCIE1B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK1_SPEC, bool, O>;
#[doc = "Field `ICIE1` reader - Timer/Counter1 Input Capture Interrupt Enable"]
pub type ICIE1_R = crate::BitReader<bool>;
#[doc = "Field `ICIE1` writer - Timer/Counter1 Input Capture Interrupt Enable"]
pub type ICIE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie1(&self) -> TOIE1_R {
        TOIE1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare A Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1a(&self) -> OCIE1A_R {
        OCIE1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare B Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1b(&self) -> OCIE1B_R {
        OCIE1B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie1(&self) -> ICIE1_R {
        ICIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie1(&mut self) -> TOIE1_W<0> {
        TOIE1_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare A Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1a(&mut self) -> OCIE1A_W<1> {
        OCIE1A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare B Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1b(&mut self) -> OCIE1B_W<2> {
        OCIE1B_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie1(&mut self) -> ICIE1_W<5> {
        ICIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter1 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk1](index.html) module"]
pub struct TIMSK1_SPEC;
impl crate::RegisterSpec for TIMSK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk1::R](R) reader structure"]
impl crate::Readable for TIMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk1::W](W) writer structure"]
impl crate::Writable for TIMSK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK1 to value 0"]
impl crate::Resettable for TIMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
