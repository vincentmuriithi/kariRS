#[doc = "Register `DACREF` reader"]
pub struct R(crate::R<DACREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACREF` writer"]
pub struct W(crate::W<DACREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACREF_SPEC>;
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
impl From<crate::W<DACREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACREF` reader - DACREF"]
pub type DACREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACREF` writer - DACREF"]
pub type DACREF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DACREF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DACREF"]
    #[inline(always)]
    pub fn dacref(&self) -> DACREF_R {
        DACREF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DACREF"]
    #[inline(always)]
    #[must_use]
    pub fn dacref(&mut self) -> DACREF_W<0> {
        DACREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "DAC Voltage Reference\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacref](index.html) module"]
pub struct DACREF_SPEC;
impl crate::RegisterSpec for DACREF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dacref::R](R) reader structure"]
impl crate::Readable for DACREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacref::W](W) writer structure"]
impl crate::Writable for DACREF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACREF to value 0"]
impl crate::Resettable for DACREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
