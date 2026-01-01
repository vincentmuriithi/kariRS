#[doc = "Register `CLKSELR` reader"]
pub struct R(crate::R<CLKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSELR` writer"]
pub struct W(crate::W<CLKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSELR_SPEC>;
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
impl From<crate::W<CLKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL` reader - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
pub type CSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSEL` writer - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
pub type CSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKSELR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSUT` reader - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
pub type CSUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSUT` writer - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
pub type CSUT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKSELR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COUT` reader - Clock Out - CKOUT fuse substitution"]
pub type COUT_R = crate::BitReader<bool>;
#[doc = "Field `COUT` writer - Clock Out - CKOUT fuse substitution"]
pub type COUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKSELR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
    #[inline(always)]
    pub fn csut(&self) -> CSUT_R {
        CSUT_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Clock Out - CKOUT fuse substitution"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
    #[inline(always)]
    #[must_use]
    pub fn csel(&mut self) -> CSEL_W<0> {
        CSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
    #[inline(always)]
    #[must_use]
    pub fn csut(&mut self) -> CSUT_W<4> {
        CSUT_W::new(self)
    }
    #[doc = "Bit 6 - Clock Out - CKOUT fuse substitution"]
    #[inline(always)]
    #[must_use]
    pub fn cout(&mut self) -> COUT_W<6> {
        COUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkselr](index.html) module"]
pub struct CLKSELR_SPEC;
impl crate::RegisterSpec for CLKSELR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkselr::R](R) reader structure"]
impl crate::Readable for CLKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkselr::W](W) writer structure"]
impl crate::Writable for CLKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSELR to value 0"]
impl crate::Resettable for CLKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
