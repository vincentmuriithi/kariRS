#[doc = "Register `OUTSET` reader"]
pub struct R(crate::R<OUTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTSET` writer"]
pub struct W(crate::W<OUTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTSET_SPEC>;
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
impl From<crate::W<OUTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB0` reader - Pin B0"]
pub type PB0_R = crate::BitReader<bool>;
#[doc = "Field `PB0` writer - Pin B0"]
pub type PB0_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTSET_SPEC, bool, O>;
#[doc = "Field `PB1` reader - Pin B1"]
pub type PB1_R = crate::BitReader<bool>;
#[doc = "Field `PB1` writer - Pin B1"]
pub type PB1_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTSET_SPEC, bool, O>;
#[doc = "Field `PB2` reader - Pin B2"]
pub type PB2_R = crate::BitReader<bool>;
#[doc = "Field `PB2` writer - Pin B2"]
pub type PB2_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTSET_SPEC, bool, O>;
#[doc = "Field `PB3` reader - Pin B3"]
pub type PB3_R = crate::BitReader<bool>;
#[doc = "Field `PB3` writer - Pin B3"]
pub type PB3_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTSET_SPEC, bool, O>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Value Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outset](index.html) module"]
pub struct OUTSET_SPEC;
impl crate::RegisterSpec for OUTSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [outset::R](R) reader structure"]
impl crate::Readable for OUTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outset::W](W) writer structure"]
impl crate::Writable for OUTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTSET to value 0"]
impl crate::Resettable for OUTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
