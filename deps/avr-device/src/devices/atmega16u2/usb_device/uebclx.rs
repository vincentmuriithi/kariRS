#[doc = "Register `UEBCLX` reader"]
pub struct R(crate::R<UEBCLX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEBCLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEBCLX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEBCLX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEBCLX` writer"]
pub struct W(crate::W<UEBCLX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEBCLX_SPEC>;
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
impl From<crate::W<UEBCLX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEBCLX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYCT` reader - Byte Count bits"]
pub type BYCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYCT` writer - Byte Count bits"]
pub type BYCT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UEBCLX_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Byte Count bits"]
    #[inline(always)]
    pub fn byct(&self) -> BYCT_R {
        BYCT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte Count bits"]
    #[inline(always)]
    #[must_use]
    pub fn byct(&mut self) -> BYCT_W<0> {
        BYCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USB Endpoint Byte Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uebclx](index.html) module"]
pub struct UEBCLX_SPEC;
impl crate::RegisterSpec for UEBCLX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uebclx::R](R) reader structure"]
impl crate::Readable for UEBCLX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uebclx::W](W) writer structure"]
impl crate::Writable for UEBCLX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEBCLX to value 0"]
impl crate::Resettable for UEBCLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
