#[doc = "Register `SCIRQM` reader"]
pub struct R(crate::R<SCIRQM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCIRQM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCIRQM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCIRQM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCIRQM` writer"]
pub struct W(crate::W<SCIRQM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCIRQM_SPEC>;
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
impl From<crate::W<SCIRQM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCIRQM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQMCP` reader - Symbol Counter Compare Match 3 IRQ enable"]
pub type IRQMCP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQMCP` writer - Symbol Counter Compare Match 3 IRQ enable"]
pub type IRQMCP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCIRQM_SPEC, u8, u8, 3, O>;
#[doc = "Field `IRQMOF` reader - Symbol Counter Overflow IRQ enable"]
pub type IRQMOF_R = crate::BitReader<bool>;
#[doc = "Field `IRQMOF` writer - Symbol Counter Overflow IRQ enable"]
pub type IRQMOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCIRQM_SPEC, bool, O>;
#[doc = "Field `IRQMBO` reader - Backoff Slot Counter IRQ enable"]
pub type IRQMBO_R = crate::BitReader<bool>;
#[doc = "Field `IRQMBO` writer - Backoff Slot Counter IRQ enable"]
pub type IRQMBO_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCIRQM_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCIRQM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Symbol Counter Compare Match 3 IRQ enable"]
    #[inline(always)]
    pub fn irqmcp(&self) -> IRQMCP_R {
        IRQMCP_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ enable"]
    #[inline(always)]
    pub fn irqmof(&self) -> IRQMOF_R {
        IRQMOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ enable"]
    #[inline(always)]
    pub fn irqmbo(&self) -> IRQMBO_R {
        IRQMBO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Symbol Counter Compare Match 3 IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqmcp(&mut self) -> IRQMCP_W<0> {
        IRQMCP_W::new(self)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqmof(&mut self) -> IRQMOF_W<3> {
        IRQMOF_W::new(self)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqmbo(&mut self) -> IRQMBO_W<4> {
        IRQMBO_W::new(self)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<5> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Symbol Counter Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scirqm](index.html) module"]
pub struct SCIRQM_SPEC;
impl crate::RegisterSpec for SCIRQM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scirqm::R](R) reader structure"]
impl crate::Readable for SCIRQM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scirqm::W](W) writer structure"]
impl crate::Writable for SCIRQM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCIRQM to value 0"]
impl crate::Resettable for SCIRQM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
