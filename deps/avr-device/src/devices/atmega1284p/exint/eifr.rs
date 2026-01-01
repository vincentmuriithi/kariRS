#[doc = "Register `EIFR` reader"]
pub struct R(crate::R<EIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIFR` writer"]
pub struct W(crate::W<EIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIFR_SPEC>;
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
impl From<crate::W<EIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTF0` reader - External Interrupt Flags 0"]
pub type INTF0_R = crate::BitReader<bool>;
#[doc = "Field `INTF0` writer - External Interrupt Flags 0"]
pub type INTF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIFR_SPEC, bool, O>;
#[doc = "Field `INTF1` reader - External Interrupt Flags 1"]
pub type INTF1_R = crate::BitReader<bool>;
#[doc = "Field `INTF1` writer - External Interrupt Flags 1"]
pub type INTF1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIFR_SPEC, bool, O>;
#[doc = "Field `INTF2` reader - External Interrupt Flags 2"]
pub type INTF2_R = crate::BitReader<bool>;
#[doc = "Field `INTF2` writer - External Interrupt Flags 2"]
pub type INTF2_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt Flags 0"]
    #[inline(always)]
    pub fn intf0(&self) -> INTF0_R {
        INTF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt Flags 1"]
    #[inline(always)]
    pub fn intf1(&self) -> INTF1_R {
        INTF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt Flags 2"]
    #[inline(always)]
    pub fn intf2(&self) -> INTF2_R {
        INTF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt Flags 0"]
    #[inline(always)]
    #[must_use]
    pub fn intf0(&mut self) -> INTF0_W<0> {
        INTF0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt Flags 1"]
    #[inline(always)]
    #[must_use]
    pub fn intf1(&mut self) -> INTF1_W<1> {
        INTF1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt Flags 2"]
    #[inline(always)]
    #[must_use]
    pub fn intf2(&mut self) -> INTF2_W<2> {
        INTF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eifr](index.html) module"]
pub struct EIFR_SPEC;
impl crate::RegisterSpec for EIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eifr::R](R) reader structure"]
impl crate::Readable for EIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eifr::W](W) writer structure"]
impl crate::Writable for EIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
