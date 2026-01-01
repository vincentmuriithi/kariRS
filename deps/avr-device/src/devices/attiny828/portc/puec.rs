#[doc = "Register `PUEC` reader"]
pub struct R(crate::R<PUEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUEC` writer"]
pub struct W(crate::W<PUEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUEC_SPEC>;
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
impl From<crate::W<PUEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUEC_SPEC>) -> Self {
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
#[doc = "Pull-up Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puec](index.html) module"]
pub struct PUEC_SPEC;
impl crate::RegisterSpec for PUEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [puec::R](R) reader structure"]
impl crate::Readable for PUEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [puec::W](W) writer structure"]
impl crate::Writable for PUEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUEC to value 0"]
impl crate::Resettable for PUEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
