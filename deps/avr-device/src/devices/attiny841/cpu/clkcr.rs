#[doc = "Register `CLKCR` reader"]
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCR` writer"]
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSEL` reader - Clock Select Bits"]
pub type CKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKSEL` writer - Clock Select Bits"]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SUT` reader - Start-up Time"]
pub type SUT_R = crate::BitReader<bool>;
#[doc = "Field `SUT` writer - Start-up Time"]
pub type SUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKCR_SPEC, bool, O>;
#[doc = "Field `CKOUTC` reader - Clock Output (Copy). Active low."]
pub type CKOUTC_R = crate::BitReader<bool>;
#[doc = "Field `CKOUTC` writer - Clock Output (Copy). Active low."]
pub type CKOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKCR_SPEC, bool, O>;
#[doc = "Field `CSTR` reader - Clock Switch Trigger"]
pub type CSTR_R = crate::BitReader<bool>;
#[doc = "Field `CSTR` writer - Clock Switch Trigger"]
pub type CSTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKCR_SPEC, bool, O>;
#[doc = "Field `OSCRDY` reader - Oscillator Ready"]
pub type OSCRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSCRDY` writer - Oscillator Ready"]
pub type OSCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Start-up Time"]
    #[inline(always)]
    pub fn sut(&self) -> SUT_R {
        SUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Output (Copy). Active low."]
    #[inline(always)]
    pub fn ckoutc(&self) -> CKOUTC_R {
        CKOUTC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Switch Trigger"]
    #[inline(always)]
    pub fn cstr(&self) -> CSTR_R {
        CSTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillator Ready"]
    #[inline(always)]
    pub fn oscrdy(&self) -> OSCRDY_R {
        OSCRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<0> {
        CKSEL_W::new(self)
    }
    #[doc = "Bit 4 - Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn sut(&mut self) -> SUT_W<4> {
        SUT_W::new(self)
    }
    #[doc = "Bit 5 - Clock Output (Copy). Active low."]
    #[inline(always)]
    #[must_use]
    pub fn ckoutc(&mut self) -> CKOUTC_W<5> {
        CKOUTC_W::new(self)
    }
    #[doc = "Bit 6 - Clock Switch Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn cstr(&mut self) -> CSTR_W<6> {
        CSTR_W::new(self)
    }
    #[doc = "Bit 7 - Oscillator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn oscrdy(&mut self) -> OSCRDY_W<7> {
        OSCRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](index.html) module"]
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkcr::R](R) reader structure"]
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcr::W](W) writer structure"]
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
