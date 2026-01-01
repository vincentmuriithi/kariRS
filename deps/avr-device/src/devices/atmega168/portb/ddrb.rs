#[doc = "Register `DDRB` reader"]
pub struct R(crate::R<DDRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRB` writer"]
pub struct W(crate::W<DDRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRB_SPEC>;
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
impl From<crate::W<DDRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB0` reader - Pin B0"]
pub type PB0_R = crate::BitReader<bool>;
#[doc = "Field `PB0` writer - Pin B0"]
pub type PB0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB1` reader - Pin B1"]
pub type PB1_R = crate::BitReader<bool>;
#[doc = "Field `PB1` writer - Pin B1"]
pub type PB1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB2` reader - Pin B2"]
pub type PB2_R = crate::BitReader<bool>;
#[doc = "Field `PB2` writer - Pin B2"]
pub type PB2_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB3` reader - Pin B3"]
pub type PB3_R = crate::BitReader<bool>;
#[doc = "Field `PB3` writer - Pin B3"]
pub type PB3_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB4` reader - Pin B4"]
pub type PB4_R = crate::BitReader<bool>;
#[doc = "Field `PB4` writer - Pin B4"]
pub type PB4_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB5` reader - Pin B5"]
pub type PB5_R = crate::BitReader<bool>;
#[doc = "Field `PB5` writer - Pin B5"]
pub type PB5_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB6` reader - Pin B6"]
pub type PB6_R = crate::BitReader<bool>;
#[doc = "Field `PB6` writer - Pin B6"]
pub type PB6_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
#[doc = "Field `PB7` reader - Pin B7"]
pub type PB7_R = crate::BitReader<bool>;
#[doc = "Field `PB7` writer - Pin B7"]
pub type PB7_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    pub fn pb0(&self) -> PB0_R {
        PB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    pub fn pb1(&self) -> PB1_R {
        PB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    pub fn pb2(&self) -> PB2_R {
        PB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    pub fn pb3(&self) -> PB3_R {
        PB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin B4"]
    #[inline(always)]
    pub fn pb4(&self) -> PB4_R {
        PB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin B5"]
    #[inline(always)]
    pub fn pb5(&self) -> PB5_R {
        PB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin B6"]
    #[inline(always)]
    pub fn pb6(&self) -> PB6_R {
        PB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin B7"]
    #[inline(always)]
    pub fn pb7(&self) -> PB7_R {
        PB7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    #[must_use]
    pub fn pb0(&mut self) -> PB0_W<0> {
        PB0_W::new(self)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    #[must_use]
    pub fn pb1(&mut self) -> PB1_W<1> {
        PB1_W::new(self)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    #[must_use]
    pub fn pb2(&mut self) -> PB2_W<2> {
        PB2_W::new(self)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    #[must_use]
    pub fn pb3(&mut self) -> PB3_W<3> {
        PB3_W::new(self)
    }
    #[doc = "Bit 4 - Pin B4"]
    #[inline(always)]
    #[must_use]
    pub fn pb4(&mut self) -> PB4_W<4> {
        PB4_W::new(self)
    }
    #[doc = "Bit 5 - Pin B5"]
    #[inline(always)]
    #[must_use]
    pub fn pb5(&mut self) -> PB5_W<5> {
        PB5_W::new(self)
    }
    #[doc = "Bit 6 - Pin B6"]
    #[inline(always)]
    #[must_use]
    pub fn pb6(&mut self) -> PB6_W<6> {
        PB6_W::new(self)
    }
    #[doc = "Bit 7 - Pin B7"]
    #[inline(always)]
    #[must_use]
    pub fn pb7(&mut self) -> PB7_W<7> {
        PB7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrb](index.html) module"]
pub struct DDRB_SPEC;
impl crate::RegisterSpec for DDRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrb::R](R) reader structure"]
impl crate::Readable for DDRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrb::W](W) writer structure"]
impl crate::Writable for DDRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRB to value 0"]
impl crate::Resettable for DDRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
