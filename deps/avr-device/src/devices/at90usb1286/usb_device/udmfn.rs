#[doc = "Register `UDMFN` reader"]
pub struct R(crate::R<UDMFN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDMFN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDMFN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDMFN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDMFN` writer"]
pub struct W(crate::W<UDMFN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDMFN_SPEC>;
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
impl From<crate::W<UDMFN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDMFN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FNCERR` reader - No Description."]
pub type FNCERR_R = crate::BitReader<bool>;
#[doc = "Field `FNCERR` writer - No Description."]
pub type FNCERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDMFN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn fncerr(&self) -> FNCERR_R {
        FNCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn fncerr(&mut self) -> FNCERR_W<4> {
        FNCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmfn](index.html) module"]
pub struct UDMFN_SPEC;
impl crate::RegisterSpec for UDMFN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [udmfn::R](R) reader structure"]
impl crate::Readable for UDMFN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udmfn::W](W) writer structure"]
impl crate::Writable for UDMFN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDMFN to value 0"]
impl crate::Resettable for UDMFN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
