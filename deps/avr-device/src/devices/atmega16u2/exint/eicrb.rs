#[doc = "Register `EICRB` reader"]
pub struct R(crate::R<EICRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EICRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EICRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EICRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EICRB` writer"]
pub struct W(crate::W<EICRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EICRB_SPEC>;
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
impl From<crate::W<EICRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EICRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC4` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC4` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRB_SPEC, u8, u8, 2, O>;
#[doc = "Field `ISC5` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC5` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRB_SPEC, u8, u8, 2, O>;
#[doc = "Field `ISC6` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC6` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC6_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRB_SPEC, u8, u8, 2, O>;
#[doc = "Field `ISC7` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISC7` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC7_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, EICRB_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc4(&mut self) -> ISC4_W<0> {
        ISC4_W::new(self)
    }
    #[doc = "Bits 2:3 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc5(&mut self) -> ISC5_W<2> {
        ISC5_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc6(&mut self) -> ISC6_W<4> {
        ISC6_W::new(self)
    }
    #[doc = "Bits 6:7 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc7(&mut self) -> ISC7_W<6> {
        ISC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eicrb](index.html) module"]
pub struct EICRB_SPEC;
impl crate::RegisterSpec for EICRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eicrb::R](R) reader structure"]
impl crate::Readable for EICRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eicrb::W](W) writer structure"]
impl crate::Writable for EICRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EICRB to value 0"]
impl crate::Resettable for EICRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
