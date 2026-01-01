#[doc = "Register `TCNT0H` reader"]
pub struct R(crate::R<TCNT0H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCNT0H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCNT0H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCNT0H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCNT0H` writer"]
pub struct W(crate::W<TCNT0H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCNT0H_SPEC>;
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
impl From<crate::W<TCNT0H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCNT0H_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter0 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcnt0h](index.html) module"]
pub struct TCNT0H_SPEC;
impl crate::RegisterSpec for TCNT0H_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcnt0h::R](R) reader structure"]
impl crate::Readable for TCNT0H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcnt0h::W](W) writer structure"]
impl crate::Writable for TCNT0H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT0H to value 0"]
impl crate::Resettable for TCNT0H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
