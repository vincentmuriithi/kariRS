#[doc = "Register `ICR1` reader"]
pub struct R(crate::R<ICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR1` writer"]
pub struct W(crate::W<ICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR1_SPEC>;
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
impl From<crate::W<ICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICR1` reader - Timer/Counter1 Input Capture bits"]
pub type ICR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ICR1` writer - Timer/Counter1 Input Capture bits"]
pub type ICR1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, ICR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 Input Capture bits"]
    #[inline(always)]
    pub fn icr1(&self) -> ICR1_R {
        ICR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 Input Capture bits"]
    #[inline(always)]
    #[must_use]
    pub fn icr1(&mut self) -> ICR1_W<0> {
        ICR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter1 Input Capture Register Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr1](index.html) module"]
pub struct ICR1_SPEC;
impl crate::RegisterSpec for ICR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [icr1::R](R) reader structure"]
impl crate::Readable for ICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr1::W](W) writer structure"]
impl crate::Writable for ICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
