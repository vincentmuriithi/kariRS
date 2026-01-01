#[doc = "Register `EPPTR` reader"]
pub struct R(crate::R<EPPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPPTR` writer"]
pub struct W(crate::W<EPPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPPTR_SPEC>;
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
impl From<crate::W<EPPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPPTR` reader - Endpoint Configuration Table Pointer"]
pub type EPPTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint Configuration Table Pointer"]
    #[inline(always)]
    pub fn epptr(&self) -> EPPTR_R {
        EPPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Endpoint Configuration Table Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epptr](index.html) module"]
pub struct EPPTR_SPEC;
impl crate::RegisterSpec for EPPTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [epptr::R](R) reader structure"]
impl crate::Readable for EPPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epptr::W](W) writer structure"]
impl crate::Writable for EPPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPPTR to value 0"]
impl crate::Resettable for EPPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
