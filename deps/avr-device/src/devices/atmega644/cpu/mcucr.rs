#[doc = "Register `MCUCR` reader"]
pub struct R(crate::R<MCUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCR` writer"]
pub struct W(crate::W<MCUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCR_SPEC>;
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
impl From<crate::W<MCUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVCE` reader - Interrupt Vector Change Enable"]
pub type IVCE_R = crate::BitReader<bool>;
#[doc = "Field `IVCE` writer - Interrupt Vector Change Enable"]
pub type IVCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader<bool>;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `PUD` reader - Pull-up disable"]
pub type PUD_R = crate::BitReader<bool>;
#[doc = "Field `PUD` writer - Pull-up disable"]
pub type PUD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `JTD` reader - JTAG Interface Disable"]
pub type JTD_R = crate::BitReader<bool>;
#[doc = "Field `JTD` writer - JTAG Interface Disable"]
pub type JTD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
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
    #[doc = "Bit 4 - Pull-up disable"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Interface Disable"]
    #[inline(always)]
    pub fn jtd(&self) -> JTD_R {
        JTD_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 4 - Pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn pud(&mut self) -> PUD_W<4> {
        PUD_W::new(self)
    }
    #[doc = "Bit 7 - JTAG Interface Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jtd(&mut self) -> JTD_W<7> {
        JTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucr](index.html) module"]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcucr::R](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucr::W](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
