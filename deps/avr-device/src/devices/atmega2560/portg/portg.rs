#[doc = "Register `PORTG` reader"]
pub struct R(crate::R<PORTG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTG` writer"]
pub struct W(crate::W<PORTG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTG_SPEC>;
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
impl From<crate::W<PORTG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG0` reader - Pin G0"]
pub type PG0_R = crate::BitReader<bool>;
#[doc = "Field `PG0` writer - Pin G0"]
pub type PG0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG1` reader - Pin G1"]
pub type PG1_R = crate::BitReader<bool>;
#[doc = "Field `PG1` writer - Pin G1"]
pub type PG1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG2` reader - Pin G2"]
pub type PG2_R = crate::BitReader<bool>;
#[doc = "Field `PG2` writer - Pin G2"]
pub type PG2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG3` reader - Pin G3"]
pub type PG3_R = crate::BitReader<bool>;
#[doc = "Field `PG3` writer - Pin G3"]
pub type PG3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG4` reader - Pin G4"]
pub type PG4_R = crate::BitReader<bool>;
#[doc = "Field `PG4` writer - Pin G4"]
pub type PG4_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG5` reader - Pin G5"]
pub type PG5_R = crate::BitReader<bool>;
#[doc = "Field `PG5` writer - Pin G5"]
pub type PG5_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG6` reader - Pin G6"]
pub type PG6_R = crate::BitReader<bool>;
#[doc = "Field `PG6` writer - Pin G6"]
pub type PG6_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
#[doc = "Field `PG7` reader - Pin G7"]
pub type PG7_R = crate::BitReader<bool>;
#[doc = "Field `PG7` writer - Pin G7"]
pub type PG7_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORTG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin G0"]
    #[inline(always)]
    pub fn pg0(&self) -> PG0_R {
        PG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin G1"]
    #[inline(always)]
    pub fn pg1(&self) -> PG1_R {
        PG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin G2"]
    #[inline(always)]
    pub fn pg2(&self) -> PG2_R {
        PG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin G3"]
    #[inline(always)]
    pub fn pg3(&self) -> PG3_R {
        PG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin G4"]
    #[inline(always)]
    pub fn pg4(&self) -> PG4_R {
        PG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin G5"]
    #[inline(always)]
    pub fn pg5(&self) -> PG5_R {
        PG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin G6"]
    #[inline(always)]
    pub fn pg6(&self) -> PG6_R {
        PG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin G7"]
    #[inline(always)]
    pub fn pg7(&self) -> PG7_R {
        PG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin G0"]
    #[inline(always)]
    #[must_use]
    pub fn pg0(&mut self) -> PG0_W<0> {
        PG0_W::new(self)
    }
    #[doc = "Bit 1 - Pin G1"]
    #[inline(always)]
    #[must_use]
    pub fn pg1(&mut self) -> PG1_W<1> {
        PG1_W::new(self)
    }
    #[doc = "Bit 2 - Pin G2"]
    #[inline(always)]
    #[must_use]
    pub fn pg2(&mut self) -> PG2_W<2> {
        PG2_W::new(self)
    }
    #[doc = "Bit 3 - Pin G3"]
    #[inline(always)]
    #[must_use]
    pub fn pg3(&mut self) -> PG3_W<3> {
        PG3_W::new(self)
    }
    #[doc = "Bit 4 - Pin G4"]
    #[inline(always)]
    #[must_use]
    pub fn pg4(&mut self) -> PG4_W<4> {
        PG4_W::new(self)
    }
    #[doc = "Bit 5 - Pin G5"]
    #[inline(always)]
    #[must_use]
    pub fn pg5(&mut self) -> PG5_W<5> {
        PG5_W::new(self)
    }
    #[doc = "Bit 6 - Pin G6"]
    #[inline(always)]
    #[must_use]
    pub fn pg6(&mut self) -> PG6_W<6> {
        PG6_W::new(self)
    }
    #[doc = "Bit 7 - Pin G7"]
    #[inline(always)]
    #[must_use]
    pub fn pg7(&mut self) -> PG7_W<7> {
        PG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Register, Port G\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portg](index.html) module"]
pub struct PORTG_SPEC;
impl crate::RegisterSpec for PORTG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [portg::R](R) reader structure"]
impl crate::Readable for PORTG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portg::W](W) writer structure"]
impl crate::Writable for PORTG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTG to value 0"]
impl crate::Resettable for PORTG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
