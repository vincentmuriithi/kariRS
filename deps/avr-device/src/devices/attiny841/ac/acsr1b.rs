#[doc = "Register `ACSR1B` reader"]
pub struct R(crate::R<ACSR1B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR1B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSR1B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSR1B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR1B` writer"]
pub struct W(crate::W<ACSR1B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR1B_SPEC>;
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
impl From<crate::W<ACSR1B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSR1B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACME1` reader - Analog Comparator 1 Multiplexer Enable"]
pub type ACME1_R = crate::BitReader<bool>;
#[doc = "Field `ACME1` writer - Analog Comparator 1 Multiplexer Enable"]
pub type ACME1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1B_SPEC, bool, O>;
#[doc = "Field `ACOE1` reader - Analog Comparator 1 Output Pin Enable"]
pub type ACOE1_R = crate::BitReader<bool>;
#[doc = "Field `ACOE1` writer - Analog Comparator 1 Output Pin Enable"]
pub type ACOE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1B_SPEC, bool, O>;
#[doc = "Field `HLEV1` reader - Analog Comparator 1 Hysteresis Level"]
pub type HLEV1_R = crate::BitReader<bool>;
#[doc = "Field `HLEV1` writer - Analog Comparator 1 Hysteresis Level"]
pub type HLEV1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1B_SPEC, bool, O>;
#[doc = "Field `HSEL1` reader - Analog Comparator 1 Hysteresis Select"]
pub type HSEL1_R = crate::BitReader<bool>;
#[doc = "Field `HSEL1` writer - Analog Comparator 1 Hysteresis Select"]
pub type HSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1B_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Analog Comparator 1 Multiplexer Enable"]
    #[inline(always)]
    pub fn acme1(&self) -> ACME1_R {
        ACME1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 1 Output Pin Enable"]
    #[inline(always)]
    pub fn acoe1(&self) -> ACOE1_R {
        ACOE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 1 Hysteresis Level"]
    #[inline(always)]
    pub fn hlev1(&self) -> HLEV1_R {
        HLEV1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 1 Hysteresis Select"]
    #[inline(always)]
    pub fn hsel1(&self) -> HSEL1_R {
        HSEL1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Analog Comparator 1 Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acme1(&mut self) -> ACME1_W<2> {
        ACME1_W::new(self)
    }
    #[doc = "Bit 4 - Analog Comparator 1 Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acoe1(&mut self) -> ACOE1_W<4> {
        ACOE1_W::new(self)
    }
    #[doc = "Bit 6 - Analog Comparator 1 Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hlev1(&mut self) -> HLEV1_W<6> {
        HLEV1_W::new(self)
    }
    #[doc = "Bit 7 - Analog Comparator 1 Hysteresis Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsel1(&mut self) -> HSEL1_W<7> {
        HSEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator 1 Control And Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr1b](index.html) module"]
pub struct ACSR1B_SPEC;
impl crate::RegisterSpec for ACSR1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acsr1b::R](R) reader structure"]
impl crate::Readable for ACSR1B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr1b::W](W) writer structure"]
impl crate::Writable for ACSR1B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR1B to value 0"]
impl crate::Resettable for ACSR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
