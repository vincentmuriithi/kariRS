#[doc = "Register `UECFG1X` reader"]
pub struct R(crate::R<UECFG1X_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UECFG1X_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UECFG1X_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UECFG1X_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UECFG1X` writer"]
pub struct W(crate::W<UECFG1X_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECFG1X_SPEC>;
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
impl From<crate::W<UECFG1X_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECFG1X_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOC` reader - Endpoint Allocation Bit"]
pub type ALLOC_R = crate::BitReader<bool>;
#[doc = "Field `ALLOC` writer - Endpoint Allocation Bit"]
pub type ALLOC_W<'a, const O: u8> = crate::BitWriter<'a, u8, UECFG1X_SPEC, bool, O>;
#[doc = "Field `EPBK` reader - Endpoint Bank Bits"]
pub type EPBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPBK` writer - Endpoint Bank Bits"]
pub type EPBK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UECFG1X_SPEC, u8, u8, 2, O>;
#[doc = "Field `EPSIZE` reader - Endpoint Size Bits"]
pub type EPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPSIZE` writer - Endpoint Size Bits"]
pub type EPSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UECFG1X_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 1 - Endpoint Allocation Bit"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Bank Bits"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:6 - Endpoint Size Bits"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Allocation Bit"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> ALLOC_W<1> {
        ALLOC_W::new(self)
    }
    #[doc = "Bits 2:3 - Endpoint Bank Bits"]
    #[inline(always)]
    #[must_use]
    pub fn epbk(&mut self) -> EPBK_W<2> {
        EPBK_W::new(self)
    }
    #[doc = "Bits 4:6 - Endpoint Size Bits"]
    #[inline(always)]
    #[must_use]
    pub fn epsize(&mut self) -> EPSIZE_W<4> {
        EPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg1x](index.html) module"]
pub struct UECFG1X_SPEC;
impl crate::RegisterSpec for UECFG1X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uecfg1x::R](R) reader structure"]
impl crate::Readable for UECFG1X_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uecfg1x::W](W) writer structure"]
impl crate::Writable for UECFG1X_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECFG1X to value 0"]
impl crate::Resettable for UECFG1X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
