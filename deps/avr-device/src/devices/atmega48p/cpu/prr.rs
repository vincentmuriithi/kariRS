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
#[doc = "Field `PRUSART0` reader - Power Reduction USART"]
pub type PRUSART0_R = crate::BitReader<bool>;
#[doc = "Field `PRUSART0` writer - Power Reduction USART"]
pub type PRUSART0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRSPI` reader - Power Reduction Serial Peripheral Interface"]
pub type PRSPI_R = crate::BitReader<bool>;
#[doc = "Field `PRSPI` writer - Power Reduction Serial Peripheral Interface"]
pub type PRSPI_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRTIM1` reader - Power Reduction Timer/Counter1"]
pub type PRTIM1_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM1` writer - Power Reduction Timer/Counter1"]
pub type PRTIM1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRTIM0` reader - Power Reduction Timer/Counter0"]
pub type PRTIM0_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM0` writer - Power Reduction Timer/Counter0"]
pub type PRTIM0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRTIM2` reader - Power Reduction Timer/Counter2"]
pub type PRTIM2_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM2` writer - Power Reduction Timer/Counter2"]
pub type PRTIM2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
#[doc = "Field `PRTWI` reader - Power Reduction TWI"]
pub type PRTWI_R = crate::BitReader<bool>;
#[doc = "Field `PRTWI` writer - Power Reduction TWI"]
pub type PRTWI_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    pub fn pradc(&self) -> PRADC_R {
        PRADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction USART"]
    #[inline(always)]
    pub fn prusart0(&self) -> PRUSART0_R {
        PRUSART0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Reduction Serial Peripheral Interface"]
    #[inline(always)]
    pub fn prspi(&self) -> PRSPI_R {
        PRSPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter1"]
    #[inline(always)]
    pub fn prtim1(&self) -> PRTIM1_R {
        PRTIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    pub fn prtim0(&self) -> PRTIM0_R {
        PRTIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power Reduction Timer/Counter2"]
    #[inline(always)]
    pub fn prtim2(&self) -> PRTIM2_R {
        PRTIM2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power Reduction TWI"]
    #[inline(always)]
    pub fn prtwi(&self) -> PRTWI_R {
        PRTWI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    #[must_use]
    pub fn pradc(&mut self) -> PRADC_W<0> {
        PRADC_W::new(self)
    }
    #[doc = "Bit 1 - Power Reduction USART"]
    #[inline(always)]
    #[must_use]
    pub fn prusart0(&mut self) -> PRUSART0_W<1> {
        PRUSART0_W::new(self)
    }
    #[doc = "Bit 2 - Power Reduction Serial Peripheral Interface"]
    #[inline(always)]
    #[must_use]
    pub fn prspi(&mut self) -> PRSPI_W<2> {
        PRSPI_W::new(self)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter1"]
    #[inline(always)]
    #[must_use]
    pub fn prtim1(&mut self) -> PRTIM1_W<3> {
        PRTIM1_W::new(self)
    }
    #[doc = "Bit 5 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn prtim0(&mut self) -> PRTIM0_W<5> {
        PRTIM0_W::new(self)
    }
    #[doc = "Bit 6 - Power Reduction Timer/Counter2"]
    #[inline(always)]
    #[must_use]
    pub fn prtim2(&mut self) -> PRTIM2_W<6> {
        PRTIM2_W::new(self)
    }
    #[doc = "Bit 7 - Power Reduction TWI"]
    #[inline(always)]
    #[must_use]
    pub fn prtwi(&mut self) -> PRTWI_W<7> {
        PRTWI_W::new(self)
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
