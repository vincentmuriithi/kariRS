#[doc = "Register `USISR` reader"]
pub struct R(crate::R<USISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USISR` writer"]
pub struct W(crate::W<USISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USISR_SPEC>;
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
impl From<crate::W<USISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USICNT` reader - USI Counter Value Bits"]
pub type USICNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USICNT` writer - USI Counter Value Bits"]
pub type USICNT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, USISR_SPEC, u8, u8, 4, O>;
#[doc = "Field `USIDC` reader - Data Output Collision"]
pub type USIDC_R = crate::BitReader<bool>;
#[doc = "Field `USIDC` writer - Data Output Collision"]
pub type USIDC_W<'a, const O: u8> = crate::BitWriter<'a, u8, USISR_SPEC, bool, O>;
#[doc = "Field `USIPF` reader - Stop Condition Flag"]
pub type USIPF_R = crate::BitReader<bool>;
#[doc = "Field `USIPF` writer - Stop Condition Flag"]
pub type USIPF_W<'a, const O: u8> = crate::BitWriter<'a, u8, USISR_SPEC, bool, O>;
#[doc = "Field `USIOIF` reader - Counter Overflow Interrupt Flag"]
pub type USIOIF_R = crate::BitReader<bool>;
#[doc = "Field `USIOIF` writer - Counter Overflow Interrupt Flag"]
pub type USIOIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, USISR_SPEC, bool, O>;
#[doc = "Field `USISIF` reader - Start Condition Interrupt Flag"]
pub type USISIF_R = crate::BitReader<bool>;
#[doc = "Field `USISIF` writer - Start Condition Interrupt Flag"]
pub type USISIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, USISR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - USI Counter Value Bits"]
    #[inline(always)]
    pub fn usicnt(&self) -> USICNT_R {
        USICNT_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Data Output Collision"]
    #[inline(always)]
    pub fn usidc(&self) -> USIDC_R {
        USIDC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Condition Flag"]
    #[inline(always)]
    pub fn usipf(&self) -> USIPF_R {
        USIPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn usioif(&self) -> USIOIF_R {
        USIOIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Flag"]
    #[inline(always)]
    pub fn usisif(&self) -> USISIF_R {
        USISIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USI Counter Value Bits"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt(&mut self) -> USICNT_W<0> {
        USICNT_W::new(self)
    }
    #[doc = "Bit 4 - Data Output Collision"]
    #[inline(always)]
    #[must_use]
    pub fn usidc(&mut self) -> USIDC_W<4> {
        USIDC_W::new(self)
    }
    #[doc = "Bit 5 - Stop Condition Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usipf(&mut self) -> USIPF_W<5> {
        USIPF_W::new(self)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usioif(&mut self) -> USIOIF_W<6> {
        USIOIF_W::new(self)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usisif(&mut self) -> USISIF_W<7> {
        USISIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usisr](index.html) module"]
pub struct USISR_SPEC;
impl crate::RegisterSpec for USISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usisr::R](R) reader structure"]
impl crate::Readable for USISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usisr::W](W) writer structure"]
impl crate::Writable for USISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USISR to value 0"]
impl crate::Resettable for USISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
