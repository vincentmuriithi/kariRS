#[doc = "Register `CTRLFSET` reader"]
pub struct R(crate::R<SINGLE_CTRLFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_CTRLFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_CTRLFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_CTRLFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLFSET` writer"]
pub struct W(crate::W<SINGLE_CTRLFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLE_CTRLFSET_SPEC>;
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
impl From<crate::W<SINGLE_CTRLFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLE_CTRLFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERBV` reader - Period Buffer Valid"]
pub type PERBV_R = crate::BitReader<bool>;
#[doc = "Field `PERBV` writer - Period Buffer Valid"]
pub type PERBV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLFSET_SPEC, bool, O>;
#[doc = "Field `CMP0BV` reader - Compare 0 Buffer Valid"]
pub type CMP0BV_R = crate::BitReader<bool>;
#[doc = "Field `CMP0BV` writer - Compare 0 Buffer Valid"]
pub type CMP0BV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLFSET_SPEC, bool, O>;
#[doc = "Field `CMP1BV` reader - Compare 1 Buffer Valid"]
pub type CMP1BV_R = crate::BitReader<bool>;
#[doc = "Field `CMP1BV` writer - Compare 1 Buffer Valid"]
pub type CMP1BV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLFSET_SPEC, bool, O>;
#[doc = "Field `CMP2BV` reader - Compare 2 Buffer Valid"]
pub type CMP2BV_R = crate::BitReader<bool>;
#[doc = "Field `CMP2BV` writer - Compare 2 Buffer Valid"]
pub type CMP2BV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLFSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&self) -> PERBV_R {
        PERBV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 0 Buffer Valid"]
    #[inline(always)]
    pub fn cmp0bv(&self) -> CMP0BV_R {
        CMP0BV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 1 Buffer Valid"]
    #[inline(always)]
    pub fn cmp1bv(&self) -> CMP1BV_R {
        CMP1BV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 2 Buffer Valid"]
    #[inline(always)]
    pub fn cmp2bv(&self) -> CMP2BV_R {
        CMP2BV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Period Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn perbv(&mut self) -> PERBV_W<0> {
        PERBV_W::new(self)
    }
    #[doc = "Bit 1 - Compare 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0bv(&mut self) -> CMP0BV_W<1> {
        CMP0BV_W::new(self)
    }
    #[doc = "Bit 2 - Compare 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1bv(&mut self) -> CMP1BV_W<2> {
        CMP1BV_W::new(self)
    }
    #[doc = "Bit 3 - Compare 2 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2bv(&mut self) -> CMP2BV_W<3> {
        CMP2BV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control F Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_ctrlfset](index.html) module"]
pub struct SINGLE_CTRLFSET_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLFSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [single_ctrlfset::R](R) reader structure"]
impl crate::Readable for SINGLE_CTRLFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_ctrlfset::W](W) writer structure"]
impl crate::Writable for SINGLE_CTRLFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLFSET to value 0"]
impl crate::Resettable for SINGLE_CTRLFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
