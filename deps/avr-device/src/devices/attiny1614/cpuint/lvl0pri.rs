#[doc = "Register `LVL0PRI` reader"]
pub struct R(crate::R<LVL0PRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVL0PRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVL0PRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVL0PRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVL0PRI` writer"]
pub struct W(crate::W<LVL0PRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVL0PRI_SPEC>;
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
impl From<crate::W<LVL0PRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVL0PRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVL0PRI` reader - Interrupt Level Priority"]
pub type LVL0PRI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVL0PRI` writer - Interrupt Level Priority"]
pub type LVL0PRI_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LVL0PRI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Level Priority"]
    #[inline(always)]
    pub fn lvl0pri(&self) -> LVL0PRI_R {
        LVL0PRI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Level Priority"]
    #[inline(always)]
    #[must_use]
    pub fn lvl0pri(&mut self) -> LVL0PRI_W<0> {
        LVL0PRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Interrupt Level 0 Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvl0pri](index.html) module"]
pub struct LVL0PRI_SPEC;
impl crate::RegisterSpec for LVL0PRI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvl0pri::R](R) reader structure"]
impl crate::Readable for LVL0PRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvl0pri::W](W) writer structure"]
impl crate::Writable for LVL0PRI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVL0PRI to value 0"]
impl crate::Resettable for LVL0PRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
