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
#[doc = "Field `ADC1D` reader - ADC2 Digital input Disable"]
pub type ADC1D_R = crate::BitReader<bool>;
#[doc = "Field `ADC1D` writer - ADC2 Digital input Disable"]
pub type ADC1D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC3D` reader - ADC3 Digital input Disable"]
pub type ADC3D_R = crate::BitReader<bool>;
#[doc = "Field `ADC3D` writer - ADC3 Digital input Disable"]
pub type ADC3D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC2D` reader - ADC2 Digital input Disable"]
pub type ADC2D_R = crate::BitReader<bool>;
#[doc = "Field `ADC2D` writer - ADC2 Digital input Disable"]
pub type ADC2D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `ADC0D` reader - ADC0 Digital input Disable"]
pub type ADC0D_R = crate::BitReader<bool>;
#[doc = "Field `ADC0D` writer - ADC0 Digital input Disable"]
pub type ADC0D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - ADC2 Digital input Disable"]
    #[inline(always)]
    pub fn adc1d(&self) -> ADC1D_R {
        ADC1D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC3 Digital input Disable"]
    #[inline(always)]
    pub fn adc3d(&self) -> ADC3D_R {
        ADC3D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC2 Digital input Disable"]
    #[inline(always)]
    pub fn adc2d(&self) -> ADC2D_R {
        ADC2D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC0 Digital input Disable"]
    #[inline(always)]
    pub fn adc0d(&self) -> ADC0D_R {
        ADC0D_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ADC2 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1d(&mut self) -> ADC1D_W<2> {
        ADC1D_W::new(self)
    }
    #[doc = "Bit 3 - ADC3 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3d(&mut self) -> ADC3D_W<3> {
        ADC3D_W::new(self)
    }
    #[doc = "Bit 4 - ADC2 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2d(&mut self) -> ADC2D_W<4> {
        ADC2D_W::new(self)
    }
    #[doc = "Bit 5 - ADC0 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0d(&mut self) -> ADC0D_W<5> {
        ADC0D_W::new(self)
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
