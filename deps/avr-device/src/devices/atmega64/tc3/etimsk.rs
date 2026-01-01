#[doc = "Register `ETIMSK` reader"]
pub struct R(crate::R<ETIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETIMSK` writer"]
pub struct W(crate::W<ETIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETIMSK_SPEC>;
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
impl From<crate::W<ETIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCIE3C` reader - Timer/Counter3, Output Compare Match Interrupt Enable"]
pub type OCIE3C_R = crate::BitReader<bool>;
#[doc = "Field `OCIE3C` writer - Timer/Counter3, Output Compare Match Interrupt Enable"]
pub type OCIE3C_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIMSK_SPEC, bool, O>;
#[doc = "Field `TOIE3` reader - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_R = crate::BitReader<bool>;
#[doc = "Field `TOIE3` writer - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIMSK_SPEC, bool, O>;
#[doc = "Field `OCIE3B` reader - Timer/Counter3 Output CompareB Match Interrupt Enable"]
pub type OCIE3B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE3B` writer - Timer/Counter3 Output CompareB Match Interrupt Enable"]
pub type OCIE3B_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIMSK_SPEC, bool, O>;
#[doc = "Field `OCIE3A` reader - Timer/Counter3 Output CompareA Match Interrupt Enable"]
pub type OCIE3A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE3A` writer - Timer/Counter3 Output CompareA Match Interrupt Enable"]
pub type OCIE3A_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIMSK_SPEC, bool, O>;
#[doc = "Field `TICIE3` reader - Timer/Counter3 Input Capture Interrupt Enable"]
pub type TICIE3_R = crate::BitReader<bool>;
#[doc = "Field `TICIE3` writer - Timer/Counter3 Input Capture Interrupt Enable"]
pub type TICIE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Timer/Counter3, Output Compare Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3c(&self) -> OCIE3C_R {
        OCIE3C_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie3(&self) -> TOIE3_R {
        TOIE3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output CompareB Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3b(&self) -> OCIE3B_R {
        OCIE3B_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter3 Output CompareA Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3a(&self) -> OCIE3A_R {
        OCIE3A_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn ticie3(&self) -> TICIE3_R {
        TICIE3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer/Counter3, Output Compare Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3c(&mut self) -> OCIE3C_W<1> {
        OCIE3C_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie3(&mut self) -> TOIE3_W<2> {
        TOIE3_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output CompareB Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3b(&mut self) -> OCIE3B_W<3> {
        OCIE3B_W::new(self)
    }
    #[doc = "Bit 4 - Timer/Counter3 Output CompareA Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3a(&mut self) -> OCIE3A_W<4> {
        OCIE3A_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ticie3(&mut self) -> TICIE3_W<5> {
        TICIE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Timer/Counter Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etimsk](index.html) module"]
pub struct ETIMSK_SPEC;
impl crate::RegisterSpec for ETIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [etimsk::R](R) reader structure"]
impl crate::Readable for ETIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etimsk::W](W) writer structure"]
impl crate::Writable for ETIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETIMSK to value 0"]
impl crate::Resettable for ETIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
