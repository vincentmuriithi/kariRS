#[doc = "Register `GICR` reader"]
pub struct R(crate::R<GICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICR` writer"]
pub struct W(crate::W<GICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICR_SPEC>;
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
impl From<crate::W<GICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVCE` reader - Interrupt Vector Change Enable"]
pub type IVCE_R = crate::BitReader<bool>;
#[doc = "Field `IVCE` writer - Interrupt Vector Change Enable"]
pub type IVCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, GICR_SPEC, bool, O>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader<bool>;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, GICR_SPEC, bool, O>;
#[doc = "Field `INT` reader - External Interrupt Request 1 Enable"]
pub type INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT` writer - External Interrupt Request 1 Enable"]
pub type INT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, GICR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    pub fn ivce(&self) -> IVCE_R {
        IVCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    pub fn ivsel(&self) -> IVSEL_R {
        IVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ivce(&mut self) -> IVCE_W<0> {
        IVCE_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<1> {
        IVSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<6> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicr](index.html) module"]
pub struct GICR_SPEC;
impl crate::RegisterSpec for GICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gicr::R](R) reader structure"]
impl crate::Readable for GICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicr::W](W) writer structure"]
impl crate::Writable for GICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICR to value 0"]
impl crate::Resettable for GICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
