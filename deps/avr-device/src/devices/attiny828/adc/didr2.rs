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
#[doc = "Field `ADC16D` reader - ADC16 Digital input Disable"]
pub type ADC16D_R = crate::BitReader<bool>;
#[doc = "Field `ADC16D` writer - ADC16 Digital input Disable"]
pub type ADC16D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC17D` reader - ADC17 Digital input Disable"]
pub type ADC17D_R = crate::BitReader<bool>;
#[doc = "Field `ADC17D` writer - ADC17 Digital input Disable"]
pub type ADC17D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC18D` reader - ADC18 Digital input Disable"]
pub type ADC18D_R = crate::BitReader<bool>;
#[doc = "Field `ADC18D` writer - ADC18 Digital input Disable"]
pub type ADC18D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC19D` reader - ADC19 Digital input Disable"]
pub type ADC19D_R = crate::BitReader<bool>;
#[doc = "Field `ADC19D` writer - ADC19 Digital input Disable"]
pub type ADC19D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC20D` reader - ADC20 Digital input Disable"]
pub type ADC20D_R = crate::BitReader<bool>;
#[doc = "Field `ADC20D` writer - ADC20 Digital input Disable"]
pub type ADC20D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC21D` reader - ADC21 Digital input Disable"]
pub type ADC21D_R = crate::BitReader<bool>;
#[doc = "Field `ADC21D` writer - ADC21 Digital input Disable"]
pub type ADC21D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC22D` reader - ADC22 Digital input Disable"]
pub type ADC22D_R = crate::BitReader<bool>;
#[doc = "Field `ADC22D` writer - ADC22 Digital input Disable"]
pub type ADC22D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
#[doc = "Field `ADC23D` reader - ADC23 Digital input Disable"]
pub type ADC23D_R = crate::BitReader<bool>;
#[doc = "Field `ADC23D` writer - ADC23 Digital input Disable"]
pub type ADC23D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC16 Digital input Disable"]
    #[inline(always)]
    pub fn adc16d(&self) -> ADC16D_R {
        ADC16D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC17 Digital input Disable"]
    #[inline(always)]
    pub fn adc17d(&self) -> ADC17D_R {
        ADC17D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC18 Digital input Disable"]
    #[inline(always)]
    pub fn adc18d(&self) -> ADC18D_R {
        ADC18D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC19 Digital input Disable"]
    #[inline(always)]
    pub fn adc19d(&self) -> ADC19D_R {
        ADC19D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC20 Digital input Disable"]
    #[inline(always)]
    pub fn adc20d(&self) -> ADC20D_R {
        ADC20D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC21 Digital input Disable"]
    #[inline(always)]
    pub fn adc21d(&self) -> ADC21D_R {
        ADC21D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC22 Digital input Disable"]
    #[inline(always)]
    pub fn adc22d(&self) -> ADC22D_R {
        ADC22D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC23 Digital input Disable"]
    #[inline(always)]
    pub fn adc23d(&self) -> ADC23D_R {
        ADC23D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC16 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc16d(&mut self) -> ADC16D_W<0> {
        ADC16D_W::new(self)
    }
    #[doc = "Bit 1 - ADC17 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc17d(&mut self) -> ADC17D_W<1> {
        ADC17D_W::new(self)
    }
    #[doc = "Bit 2 - ADC18 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc18d(&mut self) -> ADC18D_W<2> {
        ADC18D_W::new(self)
    }
    #[doc = "Bit 3 - ADC19 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc19d(&mut self) -> ADC19D_W<3> {
        ADC19D_W::new(self)
    }
    #[doc = "Bit 4 - ADC20 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc20d(&mut self) -> ADC20D_W<4> {
        ADC20D_W::new(self)
    }
    #[doc = "Bit 5 - ADC21 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc21d(&mut self) -> ADC21D_W<5> {
        ADC21D_W::new(self)
    }
    #[doc = "Bit 6 - ADC22 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc22d(&mut self) -> ADC22D_W<6> {
        ADC22D_W::new(self)
    }
    #[doc = "Bit 7 - ADC23 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc23d(&mut self) -> ADC23D_W<7> {
        ADC23D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Input Disable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr2](index.html) module"]
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
