#[doc = "Register `GIMSK` reader"]
pub struct R(crate::R<GIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GIMSK` writer"]
pub struct W(crate::W<GIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GIMSK_SPEC>;
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
impl From<crate::W<GIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCIE` reader - No Description."]
pub type PCIE_R = crate::BitReader<bool>;
#[doc = "Field `PCIE` writer - No Description."]
pub type PCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, GIMSK_SPEC, bool, O>;
#[doc = "Field `INT` reader - External Interrupt Request 1 Enable"]
pub type INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT` writer - External Interrupt Request 1 Enable"]
pub type INT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, GIMSK_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn pcie(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn pcie(&mut self) -> PCIE_W<5> {
        PCIE_W::new(self)
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
#[doc = "General Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gimsk](index.html) module"]
pub struct GIMSK_SPEC;
impl crate::RegisterSpec for GIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gimsk::R](R) reader structure"]
impl crate::Readable for GIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gimsk::W](W) writer structure"]
impl crate::Writable for GIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIMSK to value 0"]
impl crate::Resettable for GIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
