#[doc = "Register `USIDR` reader"]
pub struct R(crate::R<USIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USIDR` writer"]
pub struct W(crate::W<USIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USIDR_SPEC>;
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
impl From<crate::W<USIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USIDR_SPEC>) -> Self {
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
#[doc = "USI Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usidr](index.html) module"]
pub struct USIDR_SPEC;
impl crate::RegisterSpec for USIDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usidr::R](R) reader structure"]
impl crate::Readable for USIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usidr::W](W) writer structure"]
impl crate::Writable for USIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USIDR to value 0"]
impl crate::Resettable for USIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
