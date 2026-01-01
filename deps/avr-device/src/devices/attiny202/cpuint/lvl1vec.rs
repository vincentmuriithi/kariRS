#[doc = "Register `LVL1VEC` reader"]
pub struct R(crate::R<LVL1VEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVL1VEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVL1VEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVL1VEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVL1VEC` writer"]
pub struct W(crate::W<LVL1VEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVL1VEC_SPEC>;
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
impl From<crate::W<LVL1VEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVL1VEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVL1VEC` reader - Interrupt Vector with High Priority"]
pub type LVL1VEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVL1VEC` writer - Interrupt Vector with High Priority"]
pub type LVL1VEC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LVL1VEC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Vector with High Priority"]
    #[inline(always)]
    pub fn lvl1vec(&self) -> LVL1VEC_R {
        LVL1VEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Vector with High Priority"]
    #[inline(always)]
    #[must_use]
    pub fn lvl1vec(&mut self) -> LVL1VEC_W<0> {
        LVL1VEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Interrupt Level 1 Priority Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvl1vec](index.html) module"]
pub struct LVL1VEC_SPEC;
impl crate::RegisterSpec for LVL1VEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvl1vec::R](R) reader structure"]
impl crate::Readable for LVL1VEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvl1vec::W](W) writer structure"]
impl crate::Writable for LVL1VEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVL1VEC to value 0"]
impl crate::Resettable for LVL1VEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
