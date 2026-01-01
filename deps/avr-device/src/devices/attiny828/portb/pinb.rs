#[doc = "Register `PINB` reader"]
pub struct R(crate::R<PINB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINB` writer"]
pub struct W(crate::W<PINB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINB_SPEC>;
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
impl From<crate::W<PINB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINB_SPEC>) -> Self {
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
#[doc = "Port B Input Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinb](index.html) module"]
pub struct PINB_SPEC;
impl crate::RegisterSpec for PINB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pinb::R](R) reader structure"]
impl crate::Readable for PINB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinb::W](W) writer structure"]
impl crate::Writable for PINB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINB to value 0"]
impl crate::Resettable for PINB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
