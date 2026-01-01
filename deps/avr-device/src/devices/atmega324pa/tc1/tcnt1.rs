#[doc = "Register `TCNT1` reader"]
pub struct R(crate::R<TCNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCNT1` writer"]
pub struct W(crate::W<TCNT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCNT1_SPEC>;
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
impl From<crate::W<TCNT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCNT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT1` reader - Timer/Counter1 bits"]
pub type TCNT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TCNT1` writer - Timer/Counter1 bits"]
pub type TCNT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TCNT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 bits"]
    #[inline(always)]
    pub fn tcnt1(&self) -> TCNT1_R {
        TCNT1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt1(&mut self) -> TCNT1_W<0> {
        TCNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter1 Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcnt1](index.html) module"]
pub struct TCNT1_SPEC;
impl crate::RegisterSpec for TCNT1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcnt1::R](R) reader structure"]
impl crate::Readable for TCNT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcnt1::W](W) writer structure"]
impl crate::Writable for TCNT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT1 to value 0"]
impl crate::Resettable for TCNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
