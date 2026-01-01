#[doc = "Register `PCICR` reader"]
pub struct R(crate::R<PCICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCICR` writer"]
pub struct W(crate::W<PCICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCICR_SPEC>;
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
impl From<crate::W<PCICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCIE0` reader - Pin Change Interrupt Enable 0"]
pub type PCIE0_R = crate::BitReader<bool>;
#[doc = "Field `PCIE0` writer - Pin Change Interrupt Enable 0"]
pub type PCIE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin Change Interrupt Enable 0"]
    #[inline(always)]
    pub fn pcie0(&self) -> PCIE0_R {
        PCIE0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcie0(&mut self) -> PCIE0_W<0> {
        PCIE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Change Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcicr](index.html) module"]
pub struct PCICR_SPEC;
impl crate::RegisterSpec for PCICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcicr::R](R) reader structure"]
impl crate::Readable for PCICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcicr::W](W) writer structure"]
impl crate::Writable for PCICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCICR to value 0"]
impl crate::Resettable for PCICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
