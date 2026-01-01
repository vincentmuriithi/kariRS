#[doc = "Register `WDTCR` reader"]
pub struct R(crate::R<WDTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCR` writer"]
pub struct W(crate::W<WDTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCR_SPEC>;
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
impl From<crate::W<WDTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDPL` reader - Watchdog Timer Prescaler - Low Bits"]
pub type WDPL_R = crate::FieldReader<u8, WDPL_A>;
#[doc = "Watchdog Timer Prescaler - Low Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDPL_A {
    #[doc = "0: - 16K (16,384) cycles, ~14ms"]
    CYCLES_16K = 0,
    #[doc = "1: - 32K (32,768) cycles, ~28ms"]
    CYCLES_32K = 1,
    #[doc = "2: - 64K (65,536) cycles, ~56ms"]
    CYCLES_64K = 2,
    #[doc = "3: - 128K (131,072) cycles, ~0.11s"]
    CYCLES_128K = 3,
    #[doc = "4: - 256K (262,144) cycles, ~0.22s"]
    CYCLES_256K = 4,
    #[doc = "5: - 512K (524,288) cycles, ~0.45s"]
    CYCLES_512K = 5,
    #[doc = "6: - 1024K (1,048,576) cycles, ~0.9s"]
    CYCLES_1024K = 6,
    #[doc = "7: - 2048K (2,097,152) cycles, ~1.8s"]
    CYCLES_2048K = 7,
}
impl From<WDPL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDPL_A) -> Self {
        variant as _
    }
}
impl WDPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDPL_A {
        match self.bits {
            0 => WDPL_A::CYCLES_16K,
            1 => WDPL_A::CYCLES_32K,
            2 => WDPL_A::CYCLES_64K,
            3 => WDPL_A::CYCLES_128K,
            4 => WDPL_A::CYCLES_256K,
            5 => WDPL_A::CYCLES_512K,
            6 => WDPL_A::CYCLES_1024K,
            7 => WDPL_A::CYCLES_2048K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES_16K`"]
    #[inline(always)]
    pub fn is_cycles_16k(&self) -> bool {
        *self == WDPL_A::CYCLES_16K
    }
    #[doc = "Checks if the value of the field is `CYCLES_32K`"]
    #[inline(always)]
    pub fn is_cycles_32k(&self) -> bool {
        *self == WDPL_A::CYCLES_32K
    }
    #[doc = "Checks if the value of the field is `CYCLES_64K`"]
    #[inline(always)]
    pub fn is_cycles_64k(&self) -> bool {
        *self == WDPL_A::CYCLES_64K
    }
    #[doc = "Checks if the value of the field is `CYCLES_128K`"]
    #[inline(always)]
    pub fn is_cycles_128k(&self) -> bool {
        *self == WDPL_A::CYCLES_128K
    }
    #[doc = "Checks if the value of the field is `CYCLES_256K`"]
    #[inline(always)]
    pub fn is_cycles_256k(&self) -> bool {
        *self == WDPL_A::CYCLES_256K
    }
    #[doc = "Checks if the value of the field is `CYCLES_512K`"]
    #[inline(always)]
    pub fn is_cycles_512k(&self) -> bool {
        *self == WDPL_A::CYCLES_512K
    }
    #[doc = "Checks if the value of the field is `CYCLES_1024K`"]
    #[inline(always)]
    pub fn is_cycles_1024k(&self) -> bool {
        *self == WDPL_A::CYCLES_1024K
    }
    #[doc = "Checks if the value of the field is `CYCLES_2048K`"]
    #[inline(always)]
    pub fn is_cycles_2048k(&self) -> bool {
        *self == WDPL_A::CYCLES_2048K
    }
}
#[doc = "Field `WDPL` writer - Watchdog Timer Prescaler - Low Bits"]
pub type WDPL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, WDTCR_SPEC, u8, WDPL_A, 3, O>;
impl<'a, const O: u8> WDPL_W<'a, O> {
    #[doc = "- 16K (16,384) cycles, ~14ms"]
    #[inline(always)]
    pub fn cycles_16k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_16K)
    }
    #[doc = "- 32K (32,768) cycles, ~28ms"]
    #[inline(always)]
    pub fn cycles_32k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_32K)
    }
    #[doc = "- 64K (65,536) cycles, ~56ms"]
    #[inline(always)]
    pub fn cycles_64k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_64K)
    }
    #[doc = "- 128K (131,072) cycles, ~0.11s"]
    #[inline(always)]
    pub fn cycles_128k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_128K)
    }
    #[doc = "- 256K (262,144) cycles, ~0.22s"]
    #[inline(always)]
    pub fn cycles_256k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_256K)
    }
    #[doc = "- 512K (524,288) cycles, ~0.45s"]
    #[inline(always)]
    pub fn cycles_512k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_512K)
    }
    #[doc = "- 1024K (1,048,576) cycles, ~0.9s"]
    #[inline(always)]
    pub fn cycles_1024k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_1024K)
    }
    #[doc = "- 2048K (2,097,152) cycles, ~1.8s"]
    #[inline(always)]
    pub fn cycles_2048k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_2048K)
    }
}
#[doc = "Field `WDE` reader - Watch Dog Enable"]
pub type WDE_R = crate::BitReader<bool>;
#[doc = "Field `WDE` writer - Watch Dog Enable"]
pub type WDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCR_SPEC, bool, O>;
#[doc = "Field `WDCE` reader - Watchdog Change Enable"]
pub type WDCE_R = crate::BitReader<bool>;
#[doc = "Field `WDCE` writer - Watchdog Change Enable"]
pub type WDCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Watchdog Timer Prescaler - Low Bits"]
    #[inline(always)]
    pub fn wdpl(&self) -> WDPL_R {
        WDPL_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Watch Dog Enable"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Change Enable"]
    #[inline(always)]
    pub fn wdce(&self) -> WDCE_R {
        WDCE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watchdog Timer Prescaler - Low Bits"]
    #[inline(always)]
    #[must_use]
    pub fn wdpl(&mut self) -> WDPL_W<0> {
        WDPL_W::new(self)
    }
    #[doc = "Bit 3 - Watch Dog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WDE_W<3> {
        WDE_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdce(&mut self) -> WDCE_W<4> {
        WDCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcr](index.html) module"]
pub struct WDTCR_SPEC;
impl crate::RegisterSpec for WDTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdtcr::R](R) reader structure"]
impl crate::Readable for WDTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcr::W](W) writer structure"]
impl crate::Writable for WDTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCR to value 0"]
impl crate::Resettable for WDTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
