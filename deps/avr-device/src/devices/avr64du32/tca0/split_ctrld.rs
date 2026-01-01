#[doc = "Register `CTRLD` reader"]
pub struct R(crate::R<SPLIT_CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLIT_CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLIT_CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLIT_CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::W<SPLIT_CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLIT_CTRLD_SPEC>;
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
impl From<crate::W<SPLIT_CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLIT_CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPLITM` reader - Split Mode Enable"]
pub type SPLITM_R = crate::BitReader<bool>;
#[doc = "Field `SPLITM` writer - Split Mode Enable"]
pub type SPLITM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPLIT_CTRLD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Split Mode Enable"]
    #[inline(always)]
    pub fn splitm(&self) -> SPLITM_R {
        SPLITM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Split Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn splitm(&mut self) -> SPLITM_W<0> {
        SPLITM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [split_ctrld](index.html) module"]
pub struct SPLIT_CTRLD_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [split_ctrld::R](R) reader structure"]
impl crate::Readable for SPLIT_CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [split_ctrld::W](W) writer structure"]
impl crate::Writable for SPLIT_CTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for SPLIT_CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
