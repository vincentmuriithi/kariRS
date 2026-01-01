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
#[doc = "Field `PCIE` reader - Pin Change Interrupt Enables"]
pub type PCIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCIE` writer - Pin Change Interrupt Enables"]
pub type PCIE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, GIMSK_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT0` reader - External Interrupt Request 0 Enable"]
pub type INT0_R = crate::BitReader<bool>;
#[doc = "Field `INT0` writer - External Interrupt Request 0 Enable"]
pub type INT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, GIMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:5 - Pin Change Interrupt Enables"]
    #[inline(always)]
    pub fn pcie(&self) -> PCIE_R {
        PCIE_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - External Interrupt Request 0 Enable"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Pin Change Interrupt Enables"]
    #[inline(always)]
    #[must_use]
    pub fn pcie(&mut self) -> PCIE_W<4> {
        PCIE_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt Request 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<6> {
        INT0_W::new(self)
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
