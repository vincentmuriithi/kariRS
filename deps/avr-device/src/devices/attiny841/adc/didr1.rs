#[doc = "Register `DIDR1` reader"]
pub struct R(crate::R<DIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR1` writer"]
pub struct W(crate::W<DIDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR1_SPEC>;
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
impl From<crate::W<DIDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC11D` reader - ADC11 Digital input Disable"]
pub type ADC11D_R = crate::BitReader<bool>;
#[doc = "Field `ADC11D` writer - ADC11 Digital input Disable"]
pub type ADC11D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR1_SPEC, bool, O>;
#[doc = "Field `ADC10D` reader - ADC10 Digital input Disable"]
pub type ADC10D_R = crate::BitReader<bool>;
#[doc = "Field `ADC10D` writer - ADC10 Digital input Disable"]
pub type ADC10D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR1_SPEC, bool, O>;
#[doc = "Field `ADC8D` reader - ADC8 Digital input Disable"]
pub type ADC8D_R = crate::BitReader<bool>;
#[doc = "Field `ADC8D` writer - ADC8 Digital input Disable"]
pub type ADC8D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR1_SPEC, bool, O>;
#[doc = "Field `ADC9D` reader - ADC9 Digital Input Disable"]
pub type ADC9D_R = crate::BitReader<bool>;
#[doc = "Field `ADC9D` writer - ADC9 Digital Input Disable"]
pub type ADC9D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC11 Digital input Disable"]
    #[inline(always)]
    pub fn adc11d(&self) -> ADC11D_R {
        ADC11D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 Digital input Disable"]
    #[inline(always)]
    pub fn adc10d(&self) -> ADC10D_R {
        ADC10D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC8 Digital input Disable"]
    #[inline(always)]
    pub fn adc8d(&self) -> ADC8D_R {
        ADC8D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC9 Digital Input Disable"]
    #[inline(always)]
    pub fn adc9d(&self) -> ADC9D_R {
        ADC9D_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC11 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc11d(&mut self) -> ADC11D_W<0> {
        ADC11D_W::new(self)
    }
    #[doc = "Bit 1 - ADC10 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc10d(&mut self) -> ADC10D_W<1> {
        ADC10D_W::new(self)
    }
    #[doc = "Bit 2 - ADC8 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc8d(&mut self) -> ADC8D_W<2> {
        ADC8D_W::new(self)
    }
    #[doc = "Bit 3 - ADC9 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc9d(&mut self) -> ADC9D_W<3> {
        ADC9D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Input Disable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr1](index.html) module"]
pub struct DIDR1_SPEC;
impl crate::RegisterSpec for DIDR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [didr1::R](R) reader structure"]
impl crate::Readable for DIDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr1::W](W) writer structure"]
impl crate::Writable for DIDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR1 to value 0"]
impl crate::Resettable for DIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
