#[doc = "Register `WINHT` reader"]
pub struct R(crate::R<WINHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINHT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINHT` writer"]
pub struct W(crate::W<WINHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINHT_SPEC>;
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
impl From<crate::W<WINHT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINHT` reader - Window High Threshold"]
pub type WINHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINHT` writer - Window High Threshold"]
pub type WINHT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, WINHT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Window High Threshold"]
    #[inline(always)]
    pub fn winht(&self) -> WINHT_R {
        WINHT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winht(&mut self) -> WINHT_W<0> {
        WINHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Window comparator high threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winht](index.html) module"]
pub struct WINHT_SPEC;
impl crate::RegisterSpec for WINHT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [winht::R](R) reader structure"]
impl crate::Readable for WINHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winht::W](W) writer structure"]
impl crate::Writable for WINHT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINHT to value 0"]
impl crate::Resettable for WINHT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
