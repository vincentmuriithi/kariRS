#[doc = "Register `UESTA1X` reader"]
pub struct R(crate::R<UESTA1X_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UESTA1X_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UESTA1X_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UESTA1X_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UESTA1X` writer"]
pub struct W(crate::W<UESTA1X_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UESTA1X_SPEC>;
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
impl From<crate::W<UESTA1X_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UESTA1X_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURRBK` writer - Current Bank"]
pub type CURRBK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UESTA1X_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTRLDIR` reader - Control Direction"]
pub type CTRLDIR_R = crate::BitReader<bool>;
#[doc = "Field `CTRLDIR` writer - Control Direction"]
pub type CTRLDIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, UESTA1X_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Bank"]
    #[inline(always)]
    #[must_use]
    pub fn currbk(&mut self) -> CURRBK_W<0> {
        CURRBK_W::new(self)
    }
    #[doc = "Bit 2 - Control Direction"]
    #[inline(always)]
    #[must_use]
    pub fn ctrldir(&mut self) -> CTRLDIR_W<2> {
        CTRLDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Status 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta1x](index.html) module"]
pub struct UESTA1X_SPEC;
impl crate::RegisterSpec for UESTA1X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uesta1x::R](R) reader structure"]
impl crate::Readable for UESTA1X_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uesta1x::W](W) writer structure"]
impl crate::Writable for UESTA1X_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UESTA1X to value 0"]
impl crate::Resettable for UESTA1X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
