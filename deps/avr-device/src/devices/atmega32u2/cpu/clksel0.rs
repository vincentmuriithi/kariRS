#[doc = "Register `CLKSEL0` reader"]
pub struct R(crate::R<CLKSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSEL0` writer"]
pub struct W(crate::W<CLKSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSEL0_SPEC>;
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
impl From<crate::W<CLKSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKS` reader - No Description."]
pub type CLKS_R = crate::BitReader<bool>;
#[doc = "Field `CLKS` writer - No Description."]
pub type CLKS_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKSEL0_SPEC, bool, O>;
#[doc = "Field `EXTE` reader - No Description."]
pub type EXTE_R = crate::BitReader<bool>;
#[doc = "Field `EXTE` writer - No Description."]
pub type EXTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKSEL0_SPEC, bool, O>;
#[doc = "Field `RCE` reader - No Description."]
pub type RCE_R = crate::BitReader<bool>;
#[doc = "Field `RCE` writer - No Description."]
pub type RCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKSEL0_SPEC, bool, O>;
#[doc = "Field `EXSUT` reader - No Description."]
pub type EXSUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXSUT` writer - No Description."]
pub type EXSUT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKSEL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RCSUT` reader - No Description."]
pub type RCSUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCSUT` writer - No Description."]
pub type RCSUT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKSEL0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn exte(&self) -> EXTE_R {
        EXTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - No Description."]
    #[inline(always)]
    pub fn exsut(&self) -> EXSUT_R {
        EXSUT_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    pub fn rcsut(&self) -> RCSUT_R {
        RCSUT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> CLKS_W<0> {
        CLKS_W::new(self)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn exte(&mut self) -> EXTE_W<2> {
        EXTE_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RCE_W<3> {
        RCE_W::new(self)
    }
    #[doc = "Bits 4:5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn exsut(&mut self) -> EXSUT_W<4> {
        EXSUT_W::new(self)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rcsut(&mut self) -> RCSUT_W<6> {
        RCSUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel0](index.html) module"]
pub struct CLKSEL0_SPEC;
impl crate::RegisterSpec for CLKSEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clksel0::R](R) reader structure"]
impl crate::Readable for CLKSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksel0::W](W) writer structure"]
impl crate::Writable for CLKSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL0 to value 0"]
impl crate::Resettable for CLKSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
