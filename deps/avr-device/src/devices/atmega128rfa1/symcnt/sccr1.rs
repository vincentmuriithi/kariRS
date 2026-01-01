#[doc = "Register `SCCR1` reader"]
pub struct R(crate::R<SCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCCR1` writer"]
pub struct W(crate::W<SCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCR1_SPEC>;
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
impl From<crate::W<SCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCENBO` reader - Backoff Slot Counter enable"]
pub type SCENBO_R = crate::BitReader<bool>;
#[doc = "Field `SCENBO` writer - Backoff Slot Counter enable"]
pub type SCENBO_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCCR1_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCCR1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Backoff Slot Counter enable"]
    #[inline(always)]
    pub fn scenbo(&self) -> SCENBO_R {
        SCENBO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - Backoff Slot Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn scenbo(&mut self) -> SCENBO_W<0> {
        SCENBO_W::new(self)
    }
    #[doc = "Bits 1:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<1> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Symbol Counter Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccr1](index.html) module"]
pub struct SCCR1_SPEC;
impl crate::RegisterSpec for SCCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sccr1::R](R) reader structure"]
impl crate::Readable for SCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sccr1::W](W) writer structure"]
impl crate::Writable for SCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR1 to value 0"]
impl crate::Resettable for SCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
