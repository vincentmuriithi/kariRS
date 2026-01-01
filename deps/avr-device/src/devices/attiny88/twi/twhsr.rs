#[doc = "Register `TWHSR` reader"]
pub struct R(crate::R<TWHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWHSR` writer"]
pub struct W(crate::W<TWHSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWHSR_SPEC>;
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
impl From<crate::W<TWHSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWHSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWHS` reader - No Description."]
pub type TWHS_R = crate::BitReader<bool>;
#[doc = "Field `TWHS` writer - No Description."]
pub type TWHS_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWHSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn twhs(&self) -> TWHS_R {
        TWHS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn twhs(&mut self) -> TWHS_W<0> {
        TWHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWHSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twhsr](index.html) module"]
pub struct TWHSR_SPEC;
impl crate::RegisterSpec for TWHSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twhsr::R](R) reader structure"]
impl crate::Readable for TWHSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twhsr::W](W) writer structure"]
impl crate::Writable for TWHSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWHSR to value 0"]
impl crate::Resettable for TWHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
