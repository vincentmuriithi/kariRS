#[doc = "Register `SWEVENTA` reader"]
pub struct R(crate::R<SWEVENTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWEVENTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWEVENTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWEVENTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWEVENTA` writer"]
pub struct W(crate::W<SWEVENTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVENTA_SPEC>;
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
impl From<crate::W<SWEVENTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVENTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software event on channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWEVENTA_AW {
    #[doc = "1: Software event on channel 0"]
    CH0 = 1,
    #[doc = "2: Software event on channel 1"]
    CH1 = 2,
    #[doc = "4: Software event on channel 2"]
    CH2 = 4,
    #[doc = "8: Software event on channel 3"]
    CH3 = 8,
    #[doc = "16: Software event on channel 4"]
    CH4 = 16,
    #[doc = "32: Software event on channel 5"]
    CH5 = 32,
    #[doc = "64: Software event on channel 6"]
    CH6 = 64,
    #[doc = "128: Software event on channel 7"]
    CH7 = 128,
}
impl From<SWEVENTA_AW> for u8 {
    #[inline(always)]
    fn from(variant: SWEVENTA_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SWEVENTA` writer - Software event on channel select"]
pub type SWEVENTA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SWEVENTA_SPEC, u8, SWEVENTA_AW, 8, O>;
impl<'a, const O: u8> SWEVENTA_W<'a, O> {
    #[doc = "Software event on channel 0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH0)
    }
    #[doc = "Software event on channel 1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH1)
    }
    #[doc = "Software event on channel 2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH2)
    }
    #[doc = "Software event on channel 3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH3)
    }
    #[doc = "Software event on channel 4"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH4)
    }
    #[doc = "Software event on channel 5"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH5)
    }
    #[doc = "Software event on channel 6"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH6)
    }
    #[doc = "Software event on channel 7"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SWEVENTA_AW::CH7)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software event on channel select"]
    #[inline(always)]
    #[must_use]
    pub fn sweventa(&mut self) -> SWEVENTA_W<0> {
        SWEVENTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Event A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sweventa](index.html) module"]
pub struct SWEVENTA_SPEC;
impl crate::RegisterSpec for SWEVENTA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sweventa::R](R) reader structure"]
impl crate::Readable for SWEVENTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sweventa::W](W) writer structure"]
impl crate::Writable for SWEVENTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVENTA to value 0"]
impl crate::Resettable for SWEVENTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
