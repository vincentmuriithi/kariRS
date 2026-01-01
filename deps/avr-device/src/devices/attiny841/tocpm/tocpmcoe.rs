#[doc = "Register `TOCPMCOE` reader"]
pub struct R(crate::R<TOCPMCOE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCPMCOE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCPMCOE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCPMCOE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCPMCOE` writer"]
pub struct W(crate::W<TOCPMCOE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCPMCOE_SPEC>;
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
impl From<crate::W<TOCPMCOE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCPMCOE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCC0OE` reader - Timer Output Compare Channel 0 Output Enable"]
pub type TOCC0OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC0OE` writer - Timer Output Compare Channel 0 Output Enable"]
pub type TOCC0OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC1OE` reader - Timer Output Compare Channel 1 Output Enable"]
pub type TOCC1OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC1OE` writer - Timer Output Compare Channel 1 Output Enable"]
pub type TOCC1OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC2OE` reader - Timer Output Compare Channel 2 Output Enable"]
pub type TOCC2OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC2OE` writer - Timer Output Compare Channel 2 Output Enable"]
pub type TOCC2OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC3OE` reader - Timer Output Compare Channel 3 Output Enable"]
pub type TOCC3OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC3OE` writer - Timer Output Compare Channel 3 Output Enable"]
pub type TOCC3OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC4OE` reader - Timer Output Compare Channel 4 Output Enable"]
pub type TOCC4OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC4OE` writer - Timer Output Compare Channel 4 Output Enable"]
pub type TOCC4OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC5OE` reader - Timer Output Compare Channel 5 Output Enable"]
pub type TOCC5OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC5OE` writer - Timer Output Compare Channel 5 Output Enable"]
pub type TOCC5OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC6OE` reader - Timer Output Compare Channel 6 Output Enable"]
pub type TOCC6OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC6OE` writer - Timer Output Compare Channel 6 Output Enable"]
pub type TOCC6OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
#[doc = "Field `TOCC7OE` reader - Timer Output Compare Channel 7 Output Enable"]
pub type TOCC7OE_R = crate::BitReader<bool>;
#[doc = "Field `TOCC7OE` writer - Timer Output Compare Channel 7 Output Enable"]
pub type TOCC7OE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TOCPMCOE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer Output Compare Channel 0 Output Enable"]
    #[inline(always)]
    pub fn tocc0oe(&self) -> TOCC0OE_R {
        TOCC0OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Output Compare Channel 1 Output Enable"]
    #[inline(always)]
    pub fn tocc1oe(&self) -> TOCC1OE_R {
        TOCC1OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Output Compare Channel 2 Output Enable"]
    #[inline(always)]
    pub fn tocc2oe(&self) -> TOCC2OE_R {
        TOCC2OE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Output Compare Channel 3 Output Enable"]
    #[inline(always)]
    pub fn tocc3oe(&self) -> TOCC3OE_R {
        TOCC3OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer Output Compare Channel 4 Output Enable"]
    #[inline(always)]
    pub fn tocc4oe(&self) -> TOCC4OE_R {
        TOCC4OE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Output Compare Channel 5 Output Enable"]
    #[inline(always)]
    pub fn tocc5oe(&self) -> TOCC5OE_R {
        TOCC5OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer Output Compare Channel 6 Output Enable"]
    #[inline(always)]
    pub fn tocc6oe(&self) -> TOCC6OE_R {
        TOCC6OE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Output Compare Channel 7 Output Enable"]
    #[inline(always)]
    pub fn tocc7oe(&self) -> TOCC7OE_R {
        TOCC7OE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Output Compare Channel 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc0oe(&mut self) -> TOCC0OE_W<0> {
        TOCC0OE_W::new(self)
    }
    #[doc = "Bit 1 - Timer Output Compare Channel 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc1oe(&mut self) -> TOCC1OE_W<1> {
        TOCC1OE_W::new(self)
    }
    #[doc = "Bit 2 - Timer Output Compare Channel 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc2oe(&mut self) -> TOCC2OE_W<2> {
        TOCC2OE_W::new(self)
    }
    #[doc = "Bit 3 - Timer Output Compare Channel 3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc3oe(&mut self) -> TOCC3OE_W<3> {
        TOCC3OE_W::new(self)
    }
    #[doc = "Bit 4 - Timer Output Compare Channel 4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc4oe(&mut self) -> TOCC4OE_W<4> {
        TOCC4OE_W::new(self)
    }
    #[doc = "Bit 5 - Timer Output Compare Channel 5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc5oe(&mut self) -> TOCC5OE_W<5> {
        TOCC5OE_W::new(self)
    }
    #[doc = "Bit 6 - Timer Output Compare Channel 6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc6oe(&mut self) -> TOCC6OE_W<6> {
        TOCC6OE_W::new(self)
    }
    #[doc = "Bit 7 - Timer Output Compare Channel 7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tocc7oe(&mut self) -> TOCC7OE_W<7> {
        TOCC7OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Output Compare Pin Mux Channel Output Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocpmcoe](index.html) module"]
pub struct TOCPMCOE_SPEC;
impl crate::RegisterSpec for TOCPMCOE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tocpmcoe::R](R) reader structure"]
impl crate::Readable for TOCPMCOE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocpmcoe::W](W) writer structure"]
impl crate::Writable for TOCPMCOE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCPMCOE to value 0"]
impl crate::Resettable for TOCPMCOE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
