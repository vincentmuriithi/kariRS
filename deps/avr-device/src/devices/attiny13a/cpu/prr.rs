#[doc = "Register `PRR` reader"]
pub struct R(crate::R<PRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRR` writer"]
pub struct W(crate::W<PRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRR_SPEC>;
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
impl From<crate::W<PRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRADC` reader - Power Reduction ADC"]
pub type PRADC_R = crate::BitReader<bool>;
#[doc = "Field `PRADC` writer - Power Reduction ADC"]
pub type PRADC_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRTIM0` reader - Power Reduction Timer/Counter0"]
pub type PRTIM0_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM0` writer - Power Reduction Timer/Counter0"]
pub type PRTIM0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    pub fn pradc(&self) -> PRADC_R {
        PRADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    pub fn prtim0(&self) -> PRTIM0_R {
        PRTIM0_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    #[must_use]
    pub fn pradc(&mut self) -> PRADC_W<0> {
        PRADC_W::new(self)
    }
    #[doc = "Bit 1 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn prtim0(&mut self) -> PRTIM0_W<1> {
        PRTIM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Reduction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prr](index.html) module"]
pub struct PRR_SPEC;
impl crate::RegisterSpec for PRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prr::R](R) reader structure"]
impl crate::Readable for PRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prr::W](W) writer structure"]
impl crate::Writable for PRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR to value 0"]
impl crate::Resettable for PRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
