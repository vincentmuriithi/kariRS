#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR` reader - Error Correction Value"]
pub type ERROR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERROR` writer - Error Correction Value"]
pub type ERROR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CALIB_SPEC, u8, u8, 7, O>;
#[doc = "Field `SIGN` reader - Error Correction Sign Bit"]
pub type SIGN_R = crate::BitReader<bool>;
#[doc = "Field `SIGN` writer - Error Correction Sign Bit"]
pub type SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CALIB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Error Correction Value"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Error Correction Sign Bit"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Error Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<0> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 7 - Error Correction Sign Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<7> {
        SIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
