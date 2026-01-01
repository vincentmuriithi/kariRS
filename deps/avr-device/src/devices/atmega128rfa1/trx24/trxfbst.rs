#[doc = "Register `TRXFBST` reader"]
pub struct R(crate::R<TRXFBST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRXFBST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRXFBST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRXFBST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRXFBST` writer"]
pub struct W(crate::W<TRXFBST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRXFBST_SPEC>;
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
impl From<crate::W<TRXFBST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRXFBST_SPEC>) -> Self {
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
#[doc = "Start of frame buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trxfbst](index.html) module"]
pub struct TRXFBST_SPEC;
impl crate::RegisterSpec for TRXFBST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trxfbst::R](R) reader structure"]
impl crate::Readable for TRXFBST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trxfbst::W](W) writer structure"]
impl crate::Writable for TRXFBST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRXFBST to value 0"]
impl crate::Resettable for TRXFBST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
