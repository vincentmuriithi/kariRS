#[doc = "Register `BOOTEND` reader"]
pub struct R(crate::R<BOOTEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTEND` writer"]
pub struct W(crate::W<BOOTEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTEND_SPEC>;
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
impl From<crate::W<BOOTEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTEND_SPEC>) -> Self {
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
#[doc = "Boot Section End\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootend](index.html) module"]
pub struct BOOTEND_SPEC;
impl crate::RegisterSpec for BOOTEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bootend::R](R) reader structure"]
impl crate::Readable for BOOTEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootend::W](W) writer structure"]
impl crate::Writable for BOOTEND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTEND to value 0"]
impl crate::Resettable for BOOTEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
