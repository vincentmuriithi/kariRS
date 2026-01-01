#[doc = "Register `DIDR3` reader"]
pub struct R(crate::R<DIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR3` writer"]
pub struct W(crate::W<DIDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR3_SPEC>;
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
impl From<crate::W<DIDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC24D` reader - ADC24 Digital input Disable"]
pub type ADC24D_R = crate::BitReader<bool>;
#[doc = "Field `ADC24D` writer - ADC24 Digital input Disable"]
pub type ADC24D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR3_SPEC, bool, O>;
#[doc = "Field `ADC25D` reader - ADC25 Digital input Disable"]
pub type ADC25D_R = crate::BitReader<bool>;
#[doc = "Field `ADC25D` writer - ADC25 Digital input Disable"]
pub type ADC25D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR3_SPEC, bool, O>;
#[doc = "Field `ADC26D` reader - ADC26 Digital input Disable"]
pub type ADC26D_R = crate::BitReader<bool>;
#[doc = "Field `ADC26D` writer - ADC26 Digital input Disable"]
pub type ADC26D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR3_SPEC, bool, O>;
#[doc = "Field `ADC27D` reader - ADC27 Digital input Disable"]
pub type ADC27D_R = crate::BitReader<bool>;
#[doc = "Field `ADC27D` writer - ADC27 Digital input Disable"]
pub type ADC27D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC24 Digital input Disable"]
    #[inline(always)]
    pub fn adc24d(&self) -> ADC24D_R {
        ADC24D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC25 Digital input Disable"]
    #[inline(always)]
    pub fn adc25d(&self) -> ADC25D_R {
        ADC25D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC26 Digital input Disable"]
    #[inline(always)]
    pub fn adc26d(&self) -> ADC26D_R {
        ADC26D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC27 Digital input Disable"]
    #[inline(always)]
    pub fn adc27d(&self) -> ADC27D_R {
        ADC27D_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC24 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc24d(&mut self) -> ADC24D_W<0> {
        ADC24D_W::new(self)
    }
    #[doc = "Bit 1 - ADC25 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc25d(&mut self) -> ADC25D_W<1> {
        ADC25D_W::new(self)
    }
    #[doc = "Bit 2 - ADC26 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc26d(&mut self) -> ADC26D_W<2> {
        ADC26D_W::new(self)
    }
    #[doc = "Bit 3 - ADC27 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc27d(&mut self) -> ADC27D_W<3> {
        ADC27D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Input Disable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr3](index.html) module"]
pub struct DIDR3_SPEC;
impl crate::RegisterSpec for DIDR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [didr3::R](R) reader structure"]
impl crate::Readable for DIDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr3::W](W) writer structure"]
impl crate::Writable for DIDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR3 to value 0"]
impl crate::Resettable for DIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
