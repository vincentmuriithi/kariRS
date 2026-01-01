#[doc = "Register `PRR1` reader"]
pub struct R(crate::R<PRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRR1` writer"]
pub struct W(crate::W<PRR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRR1_SPEC>;
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
impl From<crate::W<PRR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRUSART1` reader - Power Reduction USART1"]
pub type PRUSART1_R = crate::BitReader<bool>;
#[doc = "Field `PRUSART1` writer - Power Reduction USART1"]
pub type PRUSART1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
#[doc = "Field `PRUSART2` reader - Power Reduction USART2"]
pub type PRUSART2_R = crate::BitReader<bool>;
#[doc = "Field `PRUSART2` writer - Power Reduction USART2"]
pub type PRUSART2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
#[doc = "Field `PRUSART3` reader - Power Reduction USART3"]
pub type PRUSART3_R = crate::BitReader<bool>;
#[doc = "Field `PRUSART3` writer - Power Reduction USART3"]
pub type PRUSART3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
#[doc = "Field `PRTIM3` reader - Power Reduction Timer/Counter3"]
pub type PRTIM3_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM3` writer - Power Reduction Timer/Counter3"]
pub type PRTIM3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
#[doc = "Field `PRTIM4` reader - Power Reduction Timer/Counter4"]
pub type PRTIM4_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM4` writer - Power Reduction Timer/Counter4"]
pub type PRTIM4_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
#[doc = "Field `PRTIM5` reader - Power Reduction Timer/Counter5"]
pub type PRTIM5_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM5` writer - Power Reduction Timer/Counter5"]
pub type PRTIM5_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power Reduction USART1"]
    #[inline(always)]
    pub fn prusart1(&self) -> PRUSART1_R {
        PRUSART1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction USART2"]
    #[inline(always)]
    pub fn prusart2(&self) -> PRUSART2_R {
        PRUSART2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Reduction USART3"]
    #[inline(always)]
    pub fn prusart3(&self) -> PRUSART3_R {
        PRUSART3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter3"]
    #[inline(always)]
    pub fn prtim3(&self) -> PRTIM3_R {
        PRTIM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Reduction Timer/Counter4"]
    #[inline(always)]
    pub fn prtim4(&self) -> PRTIM4_R {
        PRTIM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power Reduction Timer/Counter5"]
    #[inline(always)]
    pub fn prtim5(&self) -> PRTIM5_R {
        PRTIM5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction USART1"]
    #[inline(always)]
    #[must_use]
    pub fn prusart1(&mut self) -> PRUSART1_W<0> {
        PRUSART1_W::new(self)
    }
    #[doc = "Bit 1 - Power Reduction USART2"]
    #[inline(always)]
    #[must_use]
    pub fn prusart2(&mut self) -> PRUSART2_W<1> {
        PRUSART2_W::new(self)
    }
    #[doc = "Bit 2 - Power Reduction USART3"]
    #[inline(always)]
    #[must_use]
    pub fn prusart3(&mut self) -> PRUSART3_W<2> {
        PRUSART3_W::new(self)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter3"]
    #[inline(always)]
    #[must_use]
    pub fn prtim3(&mut self) -> PRTIM3_W<3> {
        PRTIM3_W::new(self)
    }
    #[doc = "Bit 4 - Power Reduction Timer/Counter4"]
    #[inline(always)]
    #[must_use]
    pub fn prtim4(&mut self) -> PRTIM4_W<4> {
        PRTIM4_W::new(self)
    }
    #[doc = "Bit 5 - Power Reduction Timer/Counter5"]
    #[inline(always)]
    #[must_use]
    pub fn prtim5(&mut self) -> PRTIM5_W<5> {
        PRTIM5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Reduction Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prr1](index.html) module"]
pub struct PRR1_SPEC;
impl crate::RegisterSpec for PRR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prr1::R](R) reader structure"]
impl crate::Readable for PRR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prr1::W](W) writer structure"]
impl crate::Writable for PRR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR1 to value 0"]
impl crate::Resettable for PRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
