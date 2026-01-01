#[doc = "Register `DT1` reader"]
pub struct R(crate::R<DT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT1` writer"]
pub struct W(crate::W<DT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT1_SPEC>;
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
impl From<crate::W<DT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT1L` reader - No Description."]
pub type DT1L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1L` writer - No Description."]
pub type DT1L_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DT1_SPEC, u8, u8, 4, O>;
#[doc = "Field `DT1H` reader - No Description."]
pub type DT1H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1H` writer - No Description."]
pub type DT1H_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DT1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn dt1l(&self) -> DT1L_R {
        DT1L_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    pub fn dt1h(&self) -> DT1H_R {
        DT1H_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dt1l(&mut self) -> DT1L_W<0> {
        DT1L_W::new(self)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dt1h(&mut self) -> DT1H_W<4> {
        DT1H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 1 Dead Time Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt1](index.html) module"]
pub struct DT1_SPEC;
impl crate::RegisterSpec for DT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dt1::R](R) reader structure"]
impl crate::Readable for DT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt1::W](W) writer structure"]
impl crate::Writable for DT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1 to value 0"]
impl crate::Resettable for DT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
