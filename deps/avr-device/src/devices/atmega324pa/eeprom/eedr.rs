#[doc = "Register `EEDR` reader"]
pub struct R(crate::R<EEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEDR` writer"]
pub struct W(crate::W<EEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEDR_SPEC>;
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
impl From<crate::W<EEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEDR` reader - EEPROM Data Bits"]
pub type EEDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EEDR` writer - EEPROM Data Bits"]
pub type EEDR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EEDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EEPROM Data Bits"]
    #[inline(always)]
    pub fn eedr(&self) -> EEDR_R {
        EEDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - EEPROM Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn eedr(&mut self) -> EEDR_W<0> {
        EEDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "EEPROM Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eedr](index.html) module"]
pub struct EEDR_SPEC;
impl crate::RegisterSpec for EEDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eedr::R](R) reader structure"]
impl crate::Readable for EEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eedr::W](W) writer structure"]
impl crate::Writable for EEDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEDR to value 0"]
impl crate::Resettable for EEDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
