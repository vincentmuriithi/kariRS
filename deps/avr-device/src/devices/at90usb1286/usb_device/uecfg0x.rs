#[doc = "Register `UECFG0X` reader"]
pub struct R(crate::R<UECFG0X_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UECFG0X_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UECFG0X_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UECFG0X_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UECFG0X` writer"]
pub struct W(crate::W<UECFG0X_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECFG0X_SPEC>;
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
impl From<crate::W<UECFG0X_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECFG0X_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPDIR` reader - No Description."]
pub type EPDIR_R = crate::BitReader<bool>;
#[doc = "Field `EPDIR` writer - No Description."]
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, UECFG0X_SPEC, bool, O>;
#[doc = "Field `EPTYPE` reader - No Description."]
pub type EPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYPE` writer - No Description."]
pub type EPTYPE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UECFG0X_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<0> {
        EPDIR_W::new(self)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<6> {
        EPTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg0x](index.html) module"]
pub struct UECFG0X_SPEC;
impl crate::RegisterSpec for UECFG0X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uecfg0x::R](R) reader structure"]
impl crate::Readable for UECFG0X_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uecfg0x::W](W) writer structure"]
impl crate::Writable for UECFG0X_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECFG0X to value 0"]
impl crate::Resettable for UECFG0X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
