#[doc = "Register `UEINT` reader"]
pub struct R(crate::R<UEINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEINT` writer"]
pub struct W(crate::W<UEINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEINT_SPEC>;
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
impl From<crate::W<UEINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEINT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ueint](index.html) module"]
pub struct UEINT_SPEC;
impl crate::RegisterSpec for UEINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ueint::R](R) reader structure"]
impl crate::Readable for UEINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ueint::W](W) writer structure"]
impl crate::Writable for UEINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEINT to value 0"]
impl crate::Resettable for UEINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
