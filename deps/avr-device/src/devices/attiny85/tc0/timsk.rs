#[doc = "Register `TIMSK` reader"]
pub struct R(crate::R<TIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK` writer"]
pub struct W(crate::W<TIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK_SPEC>;
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
impl From<crate::W<TIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE0` reader - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_R = crate::BitReader<bool>;
#[doc = "Field `TOIE0` writer - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK_SPEC, bool, O>;
#[doc = "Field `OCIE0B` reader - Timer/Counter0 Output Compare Match B Interrupt Enable"]
pub type OCIE0B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE0B` writer - Timer/Counter0 Output Compare Match B Interrupt Enable"]
pub type OCIE0B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK_SPEC, bool, O>;
#[doc = "Field `OCIE0A` reader - Timer/Counter0 Output Compare Match A Interrupt Enable"]
pub type OCIE0A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE0A` writer - Timer/Counter0 Output Compare Match A Interrupt Enable"]
pub type OCIE0A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie0(&self) -> TOIE0_R {
        TOIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter0 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie0b(&self) -> OCIE0B_R {
        OCIE0B_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter0 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie0a(&self) -> OCIE0A_R {
        OCIE0A_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie0(&mut self) -> TOIE0_W<1> {
        TOIE0_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter0 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie0b(&mut self) -> OCIE0B_W<3> {
        OCIE0B_W::new(self)
    }
    #[doc = "Bit 4 - Timer/Counter0 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie0a(&mut self) -> OCIE0A_W<4> {
        OCIE0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk](index.html) module"]
pub struct TIMSK_SPEC;
impl crate::RegisterSpec for TIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk::R](R) reader structure"]
impl crate::Readable for TIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk::W](W) writer structure"]
impl crate::Writable for TIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK to value 0"]
impl crate::Resettable for TIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
