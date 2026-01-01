#[doc = "Register `UDFNUM` reader"]
pub struct R(crate::R<UDFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDFNUM` writer"]
pub struct W(crate::W<UDFNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDFNUM_SPEC>;
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
impl From<crate::W<UDFNUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDFNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FNUM` reader - Frame Number Upper Flag"]
pub type FNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FNUM` writer - Frame Number Upper Flag"]
pub type FNUM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, UDFNUM_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Frame Number Upper Flag"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Number Upper Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FNUM_W<0> {
        FNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Frame Number High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udfnum](index.html) module"]
pub struct UDFNUM_SPEC;
impl crate::RegisterSpec for UDFNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [udfnum::R](R) reader structure"]
impl crate::Readable for UDFNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udfnum::W](W) writer structure"]
impl crate::Writable for UDFNUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDFNUM to value 0"]
impl crate::Resettable for UDFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
