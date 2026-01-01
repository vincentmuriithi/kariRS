#[doc = "Register `UENUM` reader"]
pub struct R(crate::R<UENUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UENUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UENUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UENUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UENUM` writer"]
pub struct W(crate::W<UENUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UENUM_SPEC>;
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
impl From<crate::W<UENUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UENUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPNUM` reader - Endpoint Number bits"]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNUM` writer - Endpoint Number bits"]
pub type EPNUM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UENUM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Endpoint Number bits"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Endpoint Number bits"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<0> {
        EPNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uenum](index.html) module"]
pub struct UENUM_SPEC;
impl crate::RegisterSpec for UENUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uenum::R](R) reader structure"]
impl crate::Readable for UENUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uenum::W](W) writer structure"]
impl crate::Writable for UENUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UENUM to value 0"]
impl crate::Resettable for UENUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
