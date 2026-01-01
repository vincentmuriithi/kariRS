#[doc = "Register `PINC` reader"]
pub struct R(crate::R<PINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINC` writer"]
pub struct W(crate::W<PINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINC_SPEC>;
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
impl From<crate::W<PINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINC_SPEC>) -> Self {
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
#[doc = "Port C Input Pins Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinc](index.html) module"]
pub struct PINC_SPEC;
impl crate::RegisterSpec for PINC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pinc::R](R) reader structure"]
impl crate::Readable for PINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinc::W](W) writer structure"]
impl crate::Writable for PINC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINC to value 0"]
impl crate::Resettable for PINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
