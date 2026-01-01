#[doc = "Register `MCUCSR` reader"]
pub struct R(crate::R<MCUCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCSR` writer"]
pub struct W(crate::W<MCUCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCSR_SPEC>;
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
impl From<crate::W<MCUCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC2` reader - Interrupt Sense Control 2"]
pub type ISC2_R = crate::BitReader<bool>;
#[doc = "Field `ISC2` writer - Interrupt Sense Control 2"]
pub type ISC2_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Interrupt Sense Control 2"]
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Interrupt Sense Control 2"]
    #[inline(always)]
    #[must_use]
    pub fn isc2(&mut self) -> ISC2_W<6> {
        ISC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Control And Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucsr](index.html) module"]
pub struct MCUCSR_SPEC;
impl crate::RegisterSpec for MCUCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcucsr::R](R) reader structure"]
impl crate::Readable for MCUCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucsr::W](W) writer structure"]
impl crate::Writable for MCUCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCSR to value 0"]
impl crate::Resettable for MCUCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
