#[doc = "Register `SYNCSTROBE` reader"]
pub struct R(crate::R<SYNCSTROBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCSTROBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCSTROBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCSTROBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCSTROBE` writer"]
pub struct W(crate::W<SYNCSTROBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCSTROBE_SPEC>;
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
impl From<crate::W<SYNCSTROBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCSTROBE_SPEC>) -> Self {
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
#[doc = "Synchronous Channel Strobe\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncstrobe](index.html) module"]
pub struct SYNCSTROBE_SPEC;
impl crate::RegisterSpec for SYNCSTROBE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syncstrobe::R](R) reader structure"]
impl crate::Readable for SYNCSTROBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncstrobe::W](W) writer structure"]
impl crate::Writable for SYNCSTROBE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCSTROBE to value 0"]
impl crate::Resettable for SYNCSTROBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
