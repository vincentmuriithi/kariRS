#[doc = "Register `EICRA` reader"]
pub struct R(crate::R<EICRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EICRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EICRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EICRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EICRA` writer"]
pub struct W(crate::W<EICRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EICRA_SPEC>;
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
impl From<crate::W<EICRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EICRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC0` reader - External Interrupt Sense Control Bit"]
pub type ISC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC0` writer - External Interrupt Sense Control Bit"]
pub type ISC0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, u8, 2, O>;
#[doc = "Field `ISC1` reader - External Interrupt Sense Control Bit"]
pub type ISC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC1` writer - External Interrupt Sense Control Bit"]
pub type ISC1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, u8, 2, O>;
#[doc = "Field `ISC2` reader - External Interrupt Sense Control Bit"]
pub type ISC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC2` writer - External Interrupt Sense Control Bit"]
pub type ISC2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, u8, 2, O>;
#[doc = "Field `ISC3` reader - External Interrupt Sense Control Bit"]
pub type ISC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC3` writer - External Interrupt Sense Control Bit"]
pub type ISC3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRA_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc3(&self) -> ISC3_R {
        ISC3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<0> {
        ISC0_W::new(self)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<2> {
        ISC1_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc2(&mut self) -> ISC2_W<4> {
        ISC2_W::new(self)
    }
    #[doc = "Bits 6:7 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc3(&mut self) -> ISC3_W<6> {
        ISC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eicra](index.html) module"]
pub struct EICRA_SPEC;
impl crate::RegisterSpec for EICRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eicra::R](R) reader structure"]
impl crate::Readable for EICRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eicra::W](W) writer structure"]
impl crate::Writable for EICRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EICRA to value 0"]
impl crate::Resettable for EICRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
