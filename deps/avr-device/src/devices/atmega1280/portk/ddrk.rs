#[doc = "Register `DDRK` reader"]
pub struct R(crate::R<DDRK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRK` writer"]
pub struct W(crate::W<DDRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRK_SPEC>;
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
impl From<crate::W<DDRK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PK0` reader - Pin K0"]
pub type PK0_R = crate::BitReader<bool>;
#[doc = "Field `PK0` writer - Pin K0"]
pub type PK0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK1` reader - Pin K1"]
pub type PK1_R = crate::BitReader<bool>;
#[doc = "Field `PK1` writer - Pin K1"]
pub type PK1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK2` reader - Pin K2"]
pub type PK2_R = crate::BitReader<bool>;
#[doc = "Field `PK2` writer - Pin K2"]
pub type PK2_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK3` reader - Pin K3"]
pub type PK3_R = crate::BitReader<bool>;
#[doc = "Field `PK3` writer - Pin K3"]
pub type PK3_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK4` reader - Pin K4"]
pub type PK4_R = crate::BitReader<bool>;
#[doc = "Field `PK4` writer - Pin K4"]
pub type PK4_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK5` reader - Pin K5"]
pub type PK5_R = crate::BitReader<bool>;
#[doc = "Field `PK5` writer - Pin K5"]
pub type PK5_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK6` reader - Pin K6"]
pub type PK6_R = crate::BitReader<bool>;
#[doc = "Field `PK6` writer - Pin K6"]
pub type PK6_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
#[doc = "Field `PK7` reader - Pin K7"]
pub type PK7_R = crate::BitReader<bool>;
#[doc = "Field `PK7` writer - Pin K7"]
pub type PK7_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin K0"]
    #[inline(always)]
    pub fn pk0(&self) -> PK0_R {
        PK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin K1"]
    #[inline(always)]
    pub fn pk1(&self) -> PK1_R {
        PK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin K2"]
    #[inline(always)]
    pub fn pk2(&self) -> PK2_R {
        PK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin K3"]
    #[inline(always)]
    pub fn pk3(&self) -> PK3_R {
        PK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin K4"]
    #[inline(always)]
    pub fn pk4(&self) -> PK4_R {
        PK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin K5"]
    #[inline(always)]
    pub fn pk5(&self) -> PK5_R {
        PK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin K6"]
    #[inline(always)]
    pub fn pk6(&self) -> PK6_R {
        PK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin K7"]
    #[inline(always)]
    pub fn pk7(&self) -> PK7_R {
        PK7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin K0"]
    #[inline(always)]
    #[must_use]
    pub fn pk0(&mut self) -> PK0_W<0> {
        PK0_W::new(self)
    }
    #[doc = "Bit 1 - Pin K1"]
    #[inline(always)]
    #[must_use]
    pub fn pk1(&mut self) -> PK1_W<1> {
        PK1_W::new(self)
    }
    #[doc = "Bit 2 - Pin K2"]
    #[inline(always)]
    #[must_use]
    pub fn pk2(&mut self) -> PK2_W<2> {
        PK2_W::new(self)
    }
    #[doc = "Bit 3 - Pin K3"]
    #[inline(always)]
    #[must_use]
    pub fn pk3(&mut self) -> PK3_W<3> {
        PK3_W::new(self)
    }
    #[doc = "Bit 4 - Pin K4"]
    #[inline(always)]
    #[must_use]
    pub fn pk4(&mut self) -> PK4_W<4> {
        PK4_W::new(self)
    }
    #[doc = "Bit 5 - Pin K5"]
    #[inline(always)]
    #[must_use]
    pub fn pk5(&mut self) -> PK5_W<5> {
        PK5_W::new(self)
    }
    #[doc = "Bit 6 - Pin K6"]
    #[inline(always)]
    #[must_use]
    pub fn pk6(&mut self) -> PK6_W<6> {
        PK6_W::new(self)
    }
    #[doc = "Bit 7 - Pin K7"]
    #[inline(always)]
    #[must_use]
    pub fn pk7(&mut self) -> PK7_W<7> {
        PK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT K Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrk](index.html) module"]
pub struct DDRK_SPEC;
impl crate::RegisterSpec for DDRK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrk::R](R) reader structure"]
impl crate::Readable for DDRK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrk::W](W) writer structure"]
impl crate::Writable for DDRK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRK to value 0"]
impl crate::Resettable for DDRK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
