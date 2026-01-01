#[doc = "Register `MBAUD` reader"]
pub struct R(crate::R<MBAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBAUD` writer"]
pub struct W(crate::W<MBAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBAUD_SPEC>;
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
impl From<crate::W<MBAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBAUD_SPEC>) -> Self {
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
#[doc = "Host Baud Rate Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbaud](index.html) module"]
pub struct MBAUD_SPEC;
impl crate::RegisterSpec for MBAUD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mbaud::R](R) reader structure"]
impl crate::Readable for MBAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbaud::W](W) writer structure"]
impl crate::Writable for MBAUD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MBAUD to value 0"]
impl crate::Resettable for MBAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
