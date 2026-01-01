#[doc = "Register `TRUTH0` reader"]
pub struct R(crate::R<TRUTH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRUTH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRUTH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRUTH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRUTH0` writer"]
pub struct W(crate::W<TRUTH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRUTH0_SPEC>;
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
impl From<crate::W<TRUTH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRUTH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRUTH` reader - Truth Table"]
pub type TRUTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRUTH` writer - Truth Table"]
pub type TRUTH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TRUTH0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Truth Table"]
    #[inline(always)]
    pub fn truth(&self) -> TRUTH_R {
        TRUTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Truth Table"]
    #[inline(always)]
    #[must_use]
    pub fn truth(&mut self) -> TRUTH_W<0> {
        TRUTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Truth 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [truth0](index.html) module"]
pub struct TRUTH0_SPEC;
impl crate::RegisterSpec for TRUTH0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [truth0::R](R) reader structure"]
impl crate::Readable for TRUTH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [truth0::W](W) writer structure"]
impl crate::Writable for TRUTH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRUTH0 to value 0"]
impl crate::Resettable for TRUTH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
