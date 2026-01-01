#[doc = "Register `PUEA` reader"]
pub struct R(crate::R<PUEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUEA` writer"]
pub struct W(crate::W<PUEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUEA_SPEC>;
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
impl From<crate::W<PUEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUEA_SPEC>) -> Self {
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
#[doc = "Pull-up Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puea](index.html) module"]
pub struct PUEA_SPEC;
impl crate::RegisterSpec for PUEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [puea::R](R) reader structure"]
impl crate::Readable for PUEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [puea::W](W) writer structure"]
impl crate::Writable for PUEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUEA to value 0"]
impl crate::Resettable for PUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
