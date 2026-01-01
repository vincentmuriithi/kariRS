#[doc = "Register `TIMSK4` reader"]
pub struct R(crate::R<TIMSK4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK4` writer"]
pub struct W(crate::W<TIMSK4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK4_SPEC>;
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
impl From<crate::W<TIMSK4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE4` reader - Timer/Counter4 Overflow Interrupt Enable"]
pub type TOIE4_R = crate::BitReader<bool>;
#[doc = "Field `TOIE4` writer - Timer/Counter4 Overflow Interrupt Enable"]
pub type TOIE4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK4_SPEC, bool, O>;
#[doc = "Field `OCIE4A` reader - Timer/Counter4 Output Compare Match A Interrupt Enable"]
pub type OCIE4A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE4A` writer - Timer/Counter4 Output Compare Match A Interrupt Enable"]
pub type OCIE4A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK4_SPEC, bool, O>;
#[doc = "Field `OCIE4B` reader - Timer/Counter4 Output Compare Match B Interrupt Enable"]
pub type OCIE4B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE4B` writer - Timer/Counter4 Output Compare Match B Interrupt Enable"]
pub type OCIE4B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK4_SPEC, bool, O>;
#[doc = "Field `ICIE4` reader - Timer/Counter4 Input Capture Interrupt Enable"]
pub type ICIE4_R = crate::BitReader<bool>;
#[doc = "Field `ICIE4` writer - Timer/Counter4 Input Capture Interrupt Enable"]
pub type ICIE4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter4 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie4(&self) -> TOIE4_R {
        TOIE4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter4 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie4a(&self) -> OCIE4A_R {
        OCIE4A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter4 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie4b(&self) -> OCIE4B_R {
        OCIE4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter4 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie4(&self) -> ICIE4_R {
        ICIE4_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter4 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie4(&mut self) -> TOIE4_W<0> {
        TOIE4_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter4 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie4a(&mut self) -> OCIE4A_W<1> {
        OCIE4A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter4 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie4b(&mut self) -> OCIE4B_W<2> {
        OCIE4B_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter4 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie4(&mut self) -> ICIE4_W<5> {
        ICIE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter4 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk4](index.html) module"]
pub struct TIMSK4_SPEC;
impl crate::RegisterSpec for TIMSK4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk4::R](R) reader structure"]
impl crate::Readable for TIMSK4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk4::W](W) writer structure"]
impl crate::Writable for TIMSK4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK4 to value 0"]
impl crate::Resettable for TIMSK4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
