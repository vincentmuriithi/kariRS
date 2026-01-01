#[doc = "Register `DIRCLR` reader"]
pub struct R(crate::R<DIRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRCLR` writer"]
pub struct W(crate::W<DIRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRCLR_SPEC>;
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
impl From<crate::W<DIRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0` reader - Pin D0"]
pub type PD0_R = crate::BitReader<bool>;
#[doc = "Field `PD0` writer - Pin D0"]
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD1` reader - Pin D1"]
pub type PD1_R = crate::BitReader<bool>;
#[doc = "Field `PD1` writer - Pin D1"]
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD2` reader - Pin D2"]
pub type PD2_R = crate::BitReader<bool>;
#[doc = "Field `PD2` writer - Pin D2"]
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD3` reader - Pin D3"]
pub type PD3_R = crate::BitReader<bool>;
#[doc = "Field `PD3` writer - Pin D3"]
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD4` reader - Pin D4"]
pub type PD4_R = crate::BitReader<bool>;
#[doc = "Field `PD4` writer - Pin D4"]
pub type PD4_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD5` reader - Pin D5"]
pub type PD5_R = crate::BitReader<bool>;
#[doc = "Field `PD5` writer - Pin D5"]
pub type PD5_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD6` reader - Pin D6"]
pub type PD6_R = crate::BitReader<bool>;
#[doc = "Field `PD6` writer - Pin D6"]
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
#[doc = "Field `PD7` reader - Pin D7"]
pub type PD7_R = crate::BitReader<bool>;
#[doc = "Field `PD7` writer - Pin D7"]
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIRCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin D0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin D1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin D2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin D3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin D4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin D0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - Pin D1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - Pin D2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - Pin D3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - Pin D4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Direction Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr](index.html) module"]
pub struct DIRCLR_SPEC;
impl crate::RegisterSpec for DIRCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dirclr::R](R) reader structure"]
impl crate::Readable for DIRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirclr::W](W) writer structure"]
impl crate::Writable for DIRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRCLR to value 0"]
impl crate::Resettable for DIRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
