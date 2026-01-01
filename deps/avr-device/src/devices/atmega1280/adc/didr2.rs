#[doc = "Register `DIDR2` reader"]
pub struct R(crate::R<DIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR2` writer"]
pub struct W(crate::W<DIDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR2_SPEC>;
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
impl From<crate::W<DIDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC8D` reader - No Description."]
pub type ADC8D_R = crate::BitReader<bool>;
#[doc = "Field `ADC8D` writer - No Description."]
pub type ADC8D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC9D` reader - No Description."]
pub type ADC9D_R = crate::BitReader<bool>;
#[doc = "Field `ADC9D` writer - No Description."]
pub type ADC9D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC10D` reader - No Description."]
pub type ADC10D_R = crate::BitReader<bool>;
#[doc = "Field `ADC10D` writer - No Description."]
pub type ADC10D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC11D` reader - No Description."]
pub type ADC11D_R = crate::BitReader<bool>;
#[doc = "Field `ADC11D` writer - No Description."]
pub type ADC11D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC12D` reader - No Description."]
pub type ADC12D_R = crate::BitReader<bool>;
#[doc = "Field `ADC12D` writer - No Description."]
pub type ADC12D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC13D` reader - No Description."]
pub type ADC13D_R = crate::BitReader<bool>;
#[doc = "Field `ADC13D` writer - No Description."]
pub type ADC13D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC14D` reader - No Description."]
pub type ADC14D_R = crate::BitReader<bool>;
#[doc = "Field `ADC14D` writer - No Description."]
pub type ADC14D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC15D` reader - No Description."]
pub type ADC15D_R = crate::BitReader<bool>;
#[doc = "Field `ADC15D` writer - No Description."]
pub type ADC15D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn adc8d(&self) -> ADC8D_R {
        ADC8D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn adc9d(&self) -> ADC9D_R {
        ADC9D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn adc10d(&self) -> ADC10D_R {
        ADC10D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn adc11d(&self) -> ADC11D_R {
        ADC11D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn adc12d(&self) -> ADC12D_R {
        ADC12D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn adc13d(&self) -> ADC13D_R {
        ADC13D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn adc14d(&self) -> ADC14D_R {
        ADC14D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn adc15d(&self) -> ADC15D_R {
        ADC15D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc8d(&mut self) -> ADC8D_W<0> {
        ADC8D_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc9d(&mut self) -> ADC9D_W<1> {
        ADC9D_W::new(self)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc10d(&mut self) -> ADC10D_W<2> {
        ADC10D_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc11d(&mut self) -> ADC11D_W<3> {
        ADC11D_W::new(self)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc12d(&mut self) -> ADC12D_W<4> {
        ADC12D_W::new(self)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc13d(&mut self) -> ADC13D_W<5> {
        ADC13D_W::new(self)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc14d(&mut self) -> ADC14D_W<6> {
        ADC14D_W::new(self)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc15d(&mut self) -> ADC15D_W<7> {
        ADC15D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Input Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr2](index.html) module"]
pub struct DIDR2_SPEC;
impl crate::RegisterSpec for DIDR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [didr2::R](R) reader structure"]
impl crate::Readable for DIDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr2::W](W) writer structure"]
impl crate::Writable for DIDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR2 to value 0"]
impl crate::Resettable for DIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
