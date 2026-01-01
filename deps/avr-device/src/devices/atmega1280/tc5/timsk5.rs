#[doc = "Register `TIMSK5` reader"]
pub struct R(crate::R<TIMSK5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK5` writer"]
pub struct W(crate::W<TIMSK5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK5_SPEC>;
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
impl From<crate::W<TIMSK5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE5` reader - Timer/Counter5 Overflow Interrupt Enable"]
pub type TOIE5_R = crate::BitReader<bool>;
#[doc = "Field `TOIE5` writer - Timer/Counter5 Overflow Interrupt Enable"]
pub type TOIE5_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK5_SPEC, bool, O>;
#[doc = "Field `OCIE5A` reader - Timer/Counter5 Output Compare A Match Interrupt Enable"]
pub type OCIE5A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE5A` writer - Timer/Counter5 Output Compare A Match Interrupt Enable"]
pub type OCIE5A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK5_SPEC, bool, O>;
#[doc = "Field `OCIE5B` reader - Timer/Counter5 Output Compare B Match Interrupt Enable"]
pub type OCIE5B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE5B` writer - Timer/Counter5 Output Compare B Match Interrupt Enable"]
pub type OCIE5B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK5_SPEC, bool, O>;
#[doc = "Field `OCIE5C` reader - Timer/Counter5 Output Compare C Match Interrupt Enable"]
pub type OCIE5C_R = crate::BitReader<bool>;
#[doc = "Field `OCIE5C` writer - Timer/Counter5 Output Compare C Match Interrupt Enable"]
pub type OCIE5C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK5_SPEC, bool, O>;
#[doc = "Field `ICIE5` reader - Timer/Counter5 Input Capture Interrupt Enable"]
pub type ICIE5_R = crate::BitReader<bool>;
#[doc = "Field `ICIE5` writer - Timer/Counter5 Input Capture Interrupt Enable"]
pub type ICIE5_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter5 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie5(&self) -> TOIE5_R {
        TOIE5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter5 Output Compare A Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie5a(&self) -> OCIE5A_R {
        OCIE5A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter5 Output Compare B Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie5b(&self) -> OCIE5B_R {
        OCIE5B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter5 Output Compare C Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie5c(&self) -> OCIE5C_R {
        OCIE5C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter5 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie5(&self) -> ICIE5_R {
        ICIE5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter5 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie5(&mut self) -> TOIE5_W<0> {
        TOIE5_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter5 Output Compare A Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie5a(&mut self) -> OCIE5A_W<1> {
        OCIE5A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter5 Output Compare B Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie5b(&mut self) -> OCIE5B_W<2> {
        OCIE5B_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter5 Output Compare C Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie5c(&mut self) -> OCIE5C_W<3> {
        OCIE5C_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter5 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie5(&mut self) -> ICIE5_W<5> {
        ICIE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter5 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk5](index.html) module"]
pub struct TIMSK5_SPEC;
impl crate::RegisterSpec for TIMSK5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk5::R](R) reader structure"]
impl crate::Readable for TIMSK5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk5::W](W) writer structure"]
impl crate::Writable for TIMSK5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK5 to value 0"]
impl crate::Resettable for TIMSK5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
