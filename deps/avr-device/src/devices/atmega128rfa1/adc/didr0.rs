#[doc = "Register `DIDR0` reader"]
pub struct R(crate::R<DIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR0` writer"]
pub struct W(crate::W<DIDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR0_SPEC>;
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
impl From<crate::W<DIDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC0D` reader - Disable ADC7:0 Digital Input"]
pub type ADC0D_R = crate::BitReader<bool>;
#[doc = "Field `ADC0D` writer - Disable ADC7:0 Digital Input"]
pub type ADC0D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC1D` reader - Disable ADC7:0 Digital Input"]
pub type ADC1D_R = crate::BitReader<bool>;
#[doc = "Field `ADC1D` writer - Disable ADC7:0 Digital Input"]
pub type ADC1D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC2D` reader - Disable ADC7:0 Digital Input"]
pub type ADC2D_R = crate::BitReader<bool>;
#[doc = "Field `ADC2D` writer - Disable ADC7:0 Digital Input"]
pub type ADC2D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC3D` reader - Disable ADC7:0 Digital Input"]
pub type ADC3D_R = crate::BitReader<bool>;
#[doc = "Field `ADC3D` writer - Disable ADC7:0 Digital Input"]
pub type ADC3D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC4D` reader - Disable ADC7:0 Digital Input"]
pub type ADC4D_R = crate::BitReader<bool>;
#[doc = "Field `ADC4D` writer - Disable ADC7:0 Digital Input"]
pub type ADC4D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC5D` reader - Disable ADC7:0 Digital Input"]
pub type ADC5D_R = crate::BitReader<bool>;
#[doc = "Field `ADC5D` writer - Disable ADC7:0 Digital Input"]
pub type ADC5D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC6D` reader - Disable ADC7:0 Digital Input"]
pub type ADC6D_R = crate::BitReader<bool>;
#[doc = "Field `ADC6D` writer - Disable ADC7:0 Digital Input"]
pub type ADC6D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC7D` reader - Disable ADC7:0 Digital Input"]
pub type ADC7D_R = crate::BitReader<bool>;
#[doc = "Field `ADC7D` writer - Disable ADC7:0 Digital Input"]
pub type ADC7D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc0d(&self) -> ADC0D_R {
        ADC0D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc1d(&self) -> ADC1D_R {
        ADC1D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc2d(&self) -> ADC2D_R {
        ADC2D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc3d(&self) -> ADC3D_R {
        ADC3D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc4d(&self) -> ADC4D_R {
        ADC4D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc5d(&self) -> ADC5D_R {
        ADC5D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc6d(&self) -> ADC6D_R {
        ADC6D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    pub fn adc7d(&self) -> ADC7D_R {
        ADC7D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc0d(&mut self) -> ADC0D_W<0> {
        ADC0D_W::new(self)
    }
    #[doc = "Bit 1 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc1d(&mut self) -> ADC1D_W<1> {
        ADC1D_W::new(self)
    }
    #[doc = "Bit 2 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc2d(&mut self) -> ADC2D_W<2> {
        ADC2D_W::new(self)
    }
    #[doc = "Bit 3 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc3d(&mut self) -> ADC3D_W<3> {
        ADC3D_W::new(self)
    }
    #[doc = "Bit 4 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc4d(&mut self) -> ADC4D_W<4> {
        ADC4D_W::new(self)
    }
    #[doc = "Bit 5 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc5d(&mut self) -> ADC5D_W<5> {
        ADC5D_W::new(self)
    }
    #[doc = "Bit 6 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc6d(&mut self) -> ADC6D_W<6> {
        ADC6D_W::new(self)
    }
    #[doc = "Bit 7 - Disable ADC7:0 Digital Input"]
    #[inline(always)]
    #[must_use]
    pub fn adc7d(&mut self) -> ADC7D_W<7> {
        ADC7D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Input Disable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr0](index.html) module"]
pub struct DIDR0_SPEC;
impl crate::RegisterSpec for DIDR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [didr0::R](R) reader structure"]
impl crate::Readable for DIDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr0::W](W) writer structure"]
impl crate::Writable for DIDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR0 to value 0"]
impl crate::Resettable for DIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
