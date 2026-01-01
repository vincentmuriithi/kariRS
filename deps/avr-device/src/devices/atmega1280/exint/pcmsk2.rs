#[doc = "Register `PCMSK2` reader"]
pub struct R(crate::R<PCMSK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMSK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMSK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMSK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMSK2` writer"]
pub struct W(crate::W<PCMSK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMSK2_SPEC>;
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
impl From<crate::W<PCMSK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMSK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINT` reader - Pin Change Enable bits"]
pub type PCINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCINT` writer - Pin Change Enable bits"]
pub type PCINT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PCMSK2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Pin Change Enable bits"]
    #[inline(always)]
    pub fn pcint(&self) -> PCINT_R {
        PCINT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin Change Enable bits"]
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
#[doc = "Pin Change Mask Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmsk2](index.html) module"]
pub struct PCMSK2_SPEC;
impl crate::RegisterSpec for PCMSK2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcmsk2::R](R) reader structure"]
impl crate::Readable for PCMSK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmsk2::W](W) writer structure"]
impl crate::Writable for PCMSK2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK2 to value 0"]
impl crate::Resettable for PCMSK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
