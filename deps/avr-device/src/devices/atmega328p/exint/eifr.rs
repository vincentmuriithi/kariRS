#[doc = "Register `EIFR` reader"]
pub struct R(crate::R<EIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIFR` writer"]
pub struct W(crate::W<EIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIFR_SPEC>;
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
impl From<crate::W<EIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTF` reader - External Interrupt Flags"]
pub type INTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTF` writer - External Interrupt Flags"]
pub type INTF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EIFR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - External Interrupt Flags"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Flags"]
    #[inline(always)]
    #[must_use]
    pub fn intf(&mut self) -> INTF_W<0> {
        INTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eifr](index.html) module"]
pub struct EIFR_SPEC;
impl crate::RegisterSpec for EIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eifr::R](R) reader structure"]
impl crate::Readable for EIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eifr::W](W) writer structure"]
impl crate::Writable for EIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
