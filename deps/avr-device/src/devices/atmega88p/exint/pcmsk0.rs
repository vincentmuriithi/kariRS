#[doc = "Register `PCMSK0` reader"]
pub struct R(crate::R<PCMSK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMSK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMSK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMSK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMSK0` writer"]
pub struct W(crate::W<PCMSK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMSK0_SPEC>;
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
impl From<crate::W<PCMSK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMSK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINT` reader - Pin Change Enable Masks"]
pub type PCINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCINT` writer - Pin Change Enable Masks"]
pub type PCINT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PCMSK0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Pin Change Enable Masks"]
    #[inline(always)]
    pub fn pcint(&self) -> PCINT_R {
        PCINT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin Change Enable Masks"]
    #[inline(always)]
    #[must_use]
    pub fn pcint(&mut self) -> PCINT_W<0> {
        PCINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Pin Change Mask Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmsk0](index.html) module"]
pub struct PCMSK0_SPEC;
impl crate::RegisterSpec for PCMSK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcmsk0::R](R) reader structure"]
impl crate::Readable for PCMSK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmsk0::W](W) writer structure"]
impl crate::Writable for PCMSK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK0 to value 0"]
impl crate::Resettable for PCMSK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
