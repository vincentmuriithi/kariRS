#[doc = "Register `TCNT3` reader"]
pub struct R(crate::R<TCNT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCNT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCNT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCNT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCNT3` writer"]
pub struct W(crate::W<TCNT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCNT3_SPEC>;
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
impl From<crate::W<TCNT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCNT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT3` reader - Timer/Counter3 bits"]
pub type TCNT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TCNT3` writer - Timer/Counter3 bits"]
pub type TCNT3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TCNT3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter3 bits"]
    #[inline(always)]
    pub fn tcnt3(&self) -> TCNT3_R {
        TCNT3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt3(&mut self) -> TCNT3_W<0> {
        TCNT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter3 Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcnt3](index.html) module"]
pub struct TCNT3_SPEC;
impl crate::RegisterSpec for TCNT3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcnt3::R](R) reader structure"]
impl crate::Readable for TCNT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcnt3::W](W) writer structure"]
impl crate::Writable for TCNT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT3 to value 0"]
impl crate::Resettable for TCNT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
