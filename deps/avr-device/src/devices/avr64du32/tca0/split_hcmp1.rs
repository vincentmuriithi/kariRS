#[doc = "Register `HCMP1` reader"]
pub struct R(crate::R<SPLIT_HCMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLIT_HCMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLIT_HCMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLIT_HCMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCMP1` writer"]
pub struct W(crate::W<SPLIT_HCMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLIT_HCMP1_SPEC>;
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
impl From<crate::W<SPLIT_HCMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLIT_HCMP1_SPEC>) -> Self {
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
#[doc = "High Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [split_hcmp1](index.html) module"]
pub struct SPLIT_HCMP1_SPEC;
impl crate::RegisterSpec for SPLIT_HCMP1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [split_hcmp1::R](R) reader structure"]
impl crate::Readable for SPLIT_HCMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [split_hcmp1::W](W) writer structure"]
impl crate::Writable for SPLIT_HCMP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCMP1 to value 0"]
impl crate::Resettable for SPLIT_HCMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
