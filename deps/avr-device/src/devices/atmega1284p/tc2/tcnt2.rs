#[doc = "Register `TCNT2` reader"]
pub struct R(crate::R<TCNT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCNT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCNT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCNT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCNT2` writer"]
pub struct W(crate::W<TCNT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCNT2_SPEC>;
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
impl From<crate::W<TCNT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCNT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT2` reader - Timer/Counter2 bits"]
pub type TCNT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCNT2` writer - Timer/Counter2 bits"]
pub type TCNT2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCNT2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter2 bits"]
    #[inline(always)]
    pub fn tcnt2(&self) -> TCNT2_R {
        TCNT2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt2(&mut self) -> TCNT2_W<0> {
        TCNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcnt2](index.html) module"]
pub struct TCNT2_SPEC;
impl crate::RegisterSpec for TCNT2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcnt2::R](R) reader structure"]
impl crate::Readable for TCNT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcnt2::W](W) writer structure"]
impl crate::Writable for TCNT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT2 to value 0"]
impl crate::Resettable for TCNT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
