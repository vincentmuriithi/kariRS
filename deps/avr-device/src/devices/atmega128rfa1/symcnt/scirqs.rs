#[doc = "Register `SCIRQS` reader"]
pub struct R(crate::R<SCIRQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCIRQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCIRQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCIRQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCIRQS` writer"]
pub struct W(crate::W<SCIRQS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCIRQS_SPEC>;
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
impl From<crate::W<SCIRQS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCIRQS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQSCP` reader - Compare Unit 3 Compare Match IRQ"]
pub type IRQSCP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQSCP` writer - Compare Unit 3 Compare Match IRQ"]
pub type IRQSCP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCIRQS_SPEC, u8, u8, 3, O>;
#[doc = "Field `IRQSOF` reader - Symbol Counter Overflow IRQ"]
pub type IRQSOF_R = crate::BitReader<bool>;
#[doc = "Field `IRQSOF` writer - Symbol Counter Overflow IRQ"]
pub type IRQSOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCIRQS_SPEC, bool, O>;
#[doc = "Field `IRQSBO` reader - Backoff Slot Counter IRQ"]
pub type IRQSBO_R = crate::BitReader<bool>;
#[doc = "Field `IRQSBO` writer - Backoff Slot Counter IRQ"]
pub type IRQSBO_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCIRQS_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCIRQS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Compare Unit 3 Compare Match IRQ"]
    #[inline(always)]
    pub fn irqscp(&self) -> IRQSCP_R {
        IRQSCP_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ"]
    #[inline(always)]
    pub fn irqsof(&self) -> IRQSOF_R {
        IRQSOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ"]
    #[inline(always)]
    pub fn irqsbo(&self) -> IRQSBO_R {
        IRQSBO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compare Unit 3 Compare Match IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn irqscp(&mut self) -> IRQSCP_W<0> {
        IRQSCP_W::new(self)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn irqsof(&mut self) -> IRQSOF_W<3> {
        IRQSOF_W::new(self)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn irqsbo(&mut self) -> IRQSBO_W<4> {
        IRQSBO_W::new(self)
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
#[doc = "Symbol Counter Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scirqs](index.html) module"]
pub struct SCIRQS_SPEC;
impl crate::RegisterSpec for SCIRQS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scirqs::R](R) reader structure"]
impl crate::Readable for SCIRQS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scirqs::W](W) writer structure"]
impl crate::Writable for SCIRQS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCIRQS to value 0"]
impl crate::Resettable for SCIRQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
