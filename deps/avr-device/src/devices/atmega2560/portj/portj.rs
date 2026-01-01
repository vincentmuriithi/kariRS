#[doc = "Register `PORTJ` reader"]
pub struct R(crate::R<PORTJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTJ` writer"]
pub struct W(crate::W<PORTJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTJ_SPEC>;
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
impl From<crate::W<PORTJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJ0` reader - Pin J0"]
pub type PJ0_R = crate::BitReader<bool>;
#[doc = "Field `PJ0` writer - Pin J0"]
pub type PJ0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ1` reader - Pin J1"]
pub type PJ1_R = crate::BitReader<bool>;
#[doc = "Field `PJ1` writer - Pin J1"]
pub type PJ1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ2` reader - Pin J2"]
pub type PJ2_R = crate::BitReader<bool>;
#[doc = "Field `PJ2` writer - Pin J2"]
pub type PJ2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ3` reader - Pin J3"]
pub type PJ3_R = crate::BitReader<bool>;
#[doc = "Field `PJ3` writer - Pin J3"]
pub type PJ3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ4` reader - Pin J4"]
pub type PJ4_R = crate::BitReader<bool>;
#[doc = "Field `PJ4` writer - Pin J4"]
pub type PJ4_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ5` reader - Pin J5"]
pub type PJ5_R = crate::BitReader<bool>;
#[doc = "Field `PJ5` writer - Pin J5"]
pub type PJ5_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ6` reader - Pin J6"]
pub type PJ6_R = crate::BitReader<bool>;
#[doc = "Field `PJ6` writer - Pin J6"]
pub type PJ6_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
#[doc = "Field `PJ7` reader - Pin J7"]
pub type PJ7_R = crate::BitReader<bool>;
#[doc = "Field `PJ7` writer - Pin J7"]
pub type PJ7_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTJ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin J0"]
    #[inline(always)]
    pub fn pj0(&self) -> PJ0_R {
        PJ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin J1"]
    #[inline(always)]
    pub fn pj1(&self) -> PJ1_R {
        PJ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin J2"]
    #[inline(always)]
    pub fn pj2(&self) -> PJ2_R {
        PJ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin J3"]
    #[inline(always)]
    pub fn pj3(&self) -> PJ3_R {
        PJ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin J4"]
    #[inline(always)]
    pub fn pj4(&self) -> PJ4_R {
        PJ4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin J5"]
    #[inline(always)]
    pub fn pj5(&self) -> PJ5_R {
        PJ5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin J6"]
    #[inline(always)]
    pub fn pj6(&self) -> PJ6_R {
        PJ6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin J7"]
    #[inline(always)]
    pub fn pj7(&self) -> PJ7_R {
        PJ7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin J0"]
    #[inline(always)]
    #[must_use]
    pub fn pj0(&mut self) -> PJ0_W<0> {
        PJ0_W::new(self)
    }
    #[doc = "Bit 1 - Pin J1"]
    #[inline(always)]
    #[must_use]
    pub fn pj1(&mut self) -> PJ1_W<1> {
        PJ1_W::new(self)
    }
    #[doc = "Bit 2 - Pin J2"]
    #[inline(always)]
    #[must_use]
    pub fn pj2(&mut self) -> PJ2_W<2> {
        PJ2_W::new(self)
    }
    #[doc = "Bit 3 - Pin J3"]
    #[inline(always)]
    #[must_use]
    pub fn pj3(&mut self) -> PJ3_W<3> {
        PJ3_W::new(self)
    }
    #[doc = "Bit 4 - Pin J4"]
    #[inline(always)]
    #[must_use]
    pub fn pj4(&mut self) -> PJ4_W<4> {
        PJ4_W::new(self)
    }
    #[doc = "Bit 5 - Pin J5"]
    #[inline(always)]
    #[must_use]
    pub fn pj5(&mut self) -> PJ5_W<5> {
        PJ5_W::new(self)
    }
    #[doc = "Bit 6 - Pin J6"]
    #[inline(always)]
    #[must_use]
    pub fn pj6(&mut self) -> PJ6_W<6> {
        PJ6_W::new(self)
    }
    #[doc = "Bit 7 - Pin J7"]
    #[inline(always)]
    #[must_use]
    pub fn pj7(&mut self) -> PJ7_W<7> {
        PJ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT J Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portj](index.html) module"]
pub struct PORTJ_SPEC;
impl crate::RegisterSpec for PORTJ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [portj::R](R) reader structure"]
impl crate::Readable for PORTJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portj::W](W) writer structure"]
impl crate::Writable for PORTJ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTJ to value 0"]
impl crate::Resettable for PORTJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
