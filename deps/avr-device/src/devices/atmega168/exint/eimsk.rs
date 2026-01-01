#[doc = "Register `EIMSK` reader"]
pub struct R(crate::R<EIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMSK` writer"]
pub struct W(crate::W<EIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMSK_SPEC>;
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
impl From<crate::W<EIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - External Interrupt Request 1 Enable"]
pub type INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT` writer - External Interrupt Request 1 Enable"]
pub type INT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EIMSK_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimsk](index.html) module"]
pub struct EIMSK_SPEC;
impl crate::RegisterSpec for EIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eimsk::R](R) reader structure"]
impl crate::Readable for EIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimsk::W](W) writer structure"]
impl crate::Writable for EIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMSK to value 0"]
impl crate::Resettable for EIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
