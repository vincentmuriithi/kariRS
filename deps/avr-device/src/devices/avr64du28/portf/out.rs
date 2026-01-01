#[doc = "Register `OUT` reader"]
pub struct R(crate::R<OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT` writer"]
pub struct W(crate::W<OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SPEC>;
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
impl From<crate::W<OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF0` reader - Pin F0"]
pub type PF0_R = crate::BitReader<bool>;
#[doc = "Field `PF0` writer - Pin F0"]
pub type PF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PF1` reader - Pin F1"]
pub type PF1_R = crate::BitReader<bool>;
#[doc = "Field `PF1` writer - Pin F1"]
pub type PF1_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PF6` reader - Pin F6"]
pub type PF6_R = crate::BitReader<bool>;
#[doc = "Field `PF6` writer - Pin F6"]
pub type PF6_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PF7` reader - Pin F7"]
pub type PF7_R = crate::BitReader<bool>;
#[doc = "Field `PF7` writer - Pin F7"]
pub type PF7_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin F0"]
    #[inline(always)]
    pub fn pf0(&self) -> PF0_R {
        PF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin F1"]
    #[inline(always)]
    pub fn pf1(&self) -> PF1_R {
        PF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    pub fn pf6(&self) -> PF6_R {
        PF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin F7"]
    #[inline(always)]
    pub fn pf7(&self) -> PF7_R {
        PF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin F0"]
    #[inline(always)]
    #[must_use]
    pub fn pf0(&mut self) -> PF0_W<0> {
        PF0_W::new(self)
    }
    #[doc = "Bit 1 - Pin F1"]
    #[inline(always)]
    #[must_use]
    pub fn pf1(&mut self) -> PF1_W<1> {
        PF1_W::new(self)
    }
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    #[must_use]
    pub fn pf6(&mut self) -> PF6_W<6> {
        PF6_W::new(self)
    }
    #[doc = "Bit 7 - Pin F7"]
    #[inline(always)]
    #[must_use]
    pub fn pf7(&mut self) -> PF7_W<7> {
        PF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](index.html) module"]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [out::R](R) reader structure"]
impl crate::Readable for OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out::W](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
