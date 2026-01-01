#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC0REFEN` reader - DAC0/AC0 reference enable"]
pub type DAC0REFEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC0REFEN` writer - DAC0/AC0 reference enable"]
pub type DAC0REFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `ADC0REFEN` reader - ADC0 reference enable"]
pub type ADC0REFEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC0REFEN` writer - ADC0 reference enable"]
pub type ADC0REFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `DAC1REFEN` reader - DAC1/AC1 reference enable"]
pub type DAC1REFEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC1REFEN` writer - DAC1/AC1 reference enable"]
pub type DAC1REFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `ADC1REFEN` reader - ADC1 reference enable"]
pub type ADC1REFEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1REFEN` writer - ADC1 reference enable"]
pub type ADC1REFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `DAC2REFEN` reader - DAC2/AC2 reference enable"]
pub type DAC2REFEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC2REFEN` writer - DAC2/AC2 reference enable"]
pub type DAC2REFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DAC0/AC0 reference enable"]
    #[inline(always)]
    pub fn dac0refen(&self) -> DAC0REFEN_R {
        DAC0REFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 reference enable"]
    #[inline(always)]
    pub fn adc0refen(&self) -> ADC0REFEN_R {
        ADC0REFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC1/AC1 reference enable"]
    #[inline(always)]
    pub fn dac1refen(&self) -> DAC1REFEN_R {
        DAC1REFEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC1 reference enable"]
    #[inline(always)]
    pub fn adc1refen(&self) -> ADC1REFEN_R {
        ADC1REFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC2/AC2 reference enable"]
    #[inline(always)]
    pub fn dac2refen(&self) -> DAC2REFEN_R {
        DAC2REFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0/AC0 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac0refen(&mut self) -> DAC0REFEN_W<0> {
        DAC0REFEN_W::new(self)
    }
    #[doc = "Bit 1 - ADC0 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0refen(&mut self) -> ADC0REFEN_W<1> {
        ADC0REFEN_W::new(self)
    }
    #[doc = "Bit 3 - DAC1/AC1 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1refen(&mut self) -> DAC1REFEN_W<3> {
        DAC1REFEN_W::new(self)
    }
    #[doc = "Bit 4 - ADC1 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1refen(&mut self) -> ADC1REFEN_W<4> {
        ADC1REFEN_W::new(self)
    }
    #[doc = "Bit 5 - DAC2/AC2 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac2refen(&mut self) -> DAC2REFEN_W<5> {
        DAC2REFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
