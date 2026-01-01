#[doc = "Register `PRR2` reader"]
pub struct R(crate::R<PRR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRR2` writer"]
pub struct W(crate::W<PRR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRR2_SPEC>;
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
impl From<crate::W<PRR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRRAM0` reader - Power Reduction SRAM0"]
pub type PRRAM0_R = crate::BitReader<bool>;
#[doc = "Field `PRRAM0` writer - Power Reduction SRAM0"]
pub type PRRAM0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR2_SPEC, bool, O>;
#[doc = "Field `PRRAM1` reader - Power Reduction SRAM1"]
pub type PRRAM1_R = crate::BitReader<bool>;
#[doc = "Field `PRRAM1` writer - Power Reduction SRAM1"]
pub type PRRAM1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR2_SPEC, bool, O>;
#[doc = "Field `PRRAM2` reader - Power Reduction SRAM2"]
pub type PRRAM2_R = crate::BitReader<bool>;
#[doc = "Field `PRRAM2` writer - Power Reduction SRAM2"]
pub type PRRAM2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR2_SPEC, bool, O>;
#[doc = "Field `PRRAM3` reader - Power Reduction SRAM3"]
pub type PRRAM3_R = crate::BitReader<bool>;
#[doc = "Field `PRRAM3` writer - Power Reduction SRAM3"]
pub type PRRAM3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power Reduction SRAM0"]
    #[inline(always)]
    pub fn prram0(&self) -> PRRAM0_R {
        PRRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction SRAM1"]
    #[inline(always)]
    pub fn prram1(&self) -> PRRAM1_R {
        PRRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Reduction SRAM2"]
    #[inline(always)]
    pub fn prram2(&self) -> PRRAM2_R {
        PRRAM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Reduction SRAM3"]
    #[inline(always)]
    pub fn prram3(&self) -> PRRAM3_R {
        PRRAM3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction SRAM0"]
    #[inline(always)]
    #[must_use]
    pub fn prram0(&mut self) -> PRRAM0_W<0> {
        PRRAM0_W::new(self)
    }
    #[doc = "Bit 1 - Power Reduction SRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn prram1(&mut self) -> PRRAM1_W<1> {
        PRRAM1_W::new(self)
    }
    #[doc = "Bit 2 - Power Reduction SRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn prram2(&mut self) -> PRRAM2_W<2> {
        PRRAM2_W::new(self)
    }
    #[doc = "Bit 3 - Power Reduction SRAM3"]
    #[inline(always)]
    #[must_use]
    pub fn prram3(&mut self) -> PRRAM3_W<3> {
        PRRAM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Reduction Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prr2](index.html) module"]
pub struct PRR2_SPEC;
impl crate::RegisterSpec for PRR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prr2::R](R) reader structure"]
impl crate::Readable for PRR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prr2::W](W) writer structure"]
impl crate::Writable for PRR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR2 to value 0"]
impl crate::Resettable for PRR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
