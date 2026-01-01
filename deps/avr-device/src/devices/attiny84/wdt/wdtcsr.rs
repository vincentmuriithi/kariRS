#[doc = "Register `WDTCSR` reader"]
pub struct R(crate::R<WDTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCSR` writer"]
pub struct W(crate::W<WDTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCSR_SPEC>;
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
impl From<crate::W<WDTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDPL` reader - Watchdog Timer Prescaler - Low Bits"]
pub type WDPL_R = crate::FieldReader<u8, WDPL_A>;
#[doc = "Watchdog Timer Prescaler - Low Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDPL_A {
    #[doc = "0: - 2048 cycles, ~16ms/512K (524288) cycles, ~4s if WDPH is set"]
    CYCLES_2K_512K = 0,
    #[doc = "1: - 4096 cycles, ~32ms/1024K (1048576) cycles, ~8s if WDPH is set"]
    CYCLES_4K_1024K = 1,
    #[doc = "2: - 8192 cycles, ~64ms"]
    CYCLES_8K = 2,
    #[doc = "3: - 16K (16384) cycles, ~0.125s"]
    CYCLES_16K = 3,
    #[doc = "4: - 32K (32768) cycles, ~0.25s"]
    CYCLES_32K = 4,
    #[doc = "5: - 64K (65536) cycles, ~0.5s"]
    CYCLES_64K = 5,
    #[doc = "6: - 128K (131072) cycles, ~1s"]
    CYCLES_128K = 6,
    #[doc = "7: - 256K (262144) cycles, ~2s"]
    CYCLES_256K = 7,
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
            0 => WDPL_A::CYCLES_2K_512K,
            1 => WDPL_A::CYCLES_4K_1024K,
            2 => WDPL_A::CYCLES_8K,
            3 => WDPL_A::CYCLES_16K,
            4 => WDPL_A::CYCLES_32K,
            5 => WDPL_A::CYCLES_64K,
            6 => WDPL_A::CYCLES_128K,
            7 => WDPL_A::CYCLES_256K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES_2K_512K`"]
    #[inline(always)]
    pub fn is_cycles_2k_512k(&self) -> bool {
        *self == WDPL_A::CYCLES_2K_512K
    }
    #[doc = "Checks if the value of the field is `CYCLES_4K_1024K`"]
    #[inline(always)]
    pub fn is_cycles_4k_1024k(&self) -> bool {
        *self == WDPL_A::CYCLES_4K_1024K
    }
    #[doc = "Checks if the value of the field is `CYCLES_8K`"]
    #[inline(always)]
    pub fn is_cycles_8k(&self) -> bool {
        *self == WDPL_A::CYCLES_8K
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
}
#[doc = "Field `WDPL` writer - Watchdog Timer Prescaler - Low Bits"]
pub type WDPL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, WDTCSR_SPEC, u8, WDPL_A, 3, O>;
impl<'a, const O: u8> WDPL_W<'a, O> {
    #[doc = "- 2048 cycles, ~16ms/512K (524288) cycles, ~4s if WDPH is set"]
    #[inline(always)]
    pub fn cycles_2k_512k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_2K_512K)
    }
    #[doc = "- 4096 cycles, ~32ms/1024K (1048576) cycles, ~8s if WDPH is set"]
    #[inline(always)]
    pub fn cycles_4k_1024k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_4K_1024K)
    }
    #[doc = "- 8192 cycles, ~64ms"]
    #[inline(always)]
    pub fn cycles_8k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_8K)
    }
    #[doc = "- 16K (16384) cycles, ~0.125s"]
    #[inline(always)]
    pub fn cycles_16k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_16K)
    }
    #[doc = "- 32K (32768) cycles, ~0.25s"]
    #[inline(always)]
    pub fn cycles_32k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_32K)
    }
    #[doc = "- 64K (65536) cycles, ~0.5s"]
    #[inline(always)]
    pub fn cycles_64k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_64K)
    }
    #[doc = "- 128K (131072) cycles, ~1s"]
    #[inline(always)]
    pub fn cycles_128k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_128K)
    }
    #[doc = "- 256K (262144) cycles, ~2s"]
    #[inline(always)]
    pub fn cycles_256k(self) -> &'a mut W {
        self.variant(WDPL_A::CYCLES_256K)
    }
}
#[doc = "Field `WDE` reader - Watch Dog Enable"]
pub type WDE_R = crate::BitReader<bool>;
#[doc = "Field `WDE` writer - Watch Dog Enable"]
pub type WDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
#[doc = "Field `WDCE` reader - Watchdog Change Enable"]
pub type WDCE_R = crate::BitReader<bool>;
#[doc = "Field `WDCE` writer - Watchdog Change Enable"]
pub type WDCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
#[doc = "Field `WDPH` reader - Watchdog Timer Prescaler - High Bit"]
pub type WDPH_R = crate::BitReader<bool>;
#[doc = "Field `WDPH` writer - Watchdog Timer Prescaler - High Bit"]
pub type WDPH_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
#[doc = "Field `WDIE` reader - Watchdog Timeout Interrupt Enable"]
pub type WDIE_R = crate::BitReader<bool>;
#[doc = "Field `WDIE` writer - Watchdog Timeout Interrupt Enable"]
pub type WDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
#[doc = "Field `WDIF` reader - Watchdog Timeout Interrupt Flag"]
pub type WDIF_R = crate::BitReader<bool>;
#[doc = "Field `WDIF` writer - Watchdog Timeout Interrupt Flag"]
pub type WDIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
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
    #[doc = "Bit 5 - Watchdog Timer Prescaler - High Bit"]
    #[inline(always)]
    pub fn wdph(&self) -> WDPH_R {
        WDPH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Watchdog Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn wdif(&self) -> WDIF_R {
        WDIF_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 5 - Watchdog Timer Prescaler - High Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdph(&mut self) -> WDPH_W<5> {
        WDPH_W::new(self)
    }
    #[doc = "Bit 6 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<6> {
        WDIE_W::new(self)
    }
    #[doc = "Bit 7 - Watchdog Timeout Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdif(&mut self) -> WDIF_W<7> {
        WDIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcsr](index.html) module"]
pub struct WDTCSR_SPEC;
impl crate::RegisterSpec for WDTCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdtcsr::R](R) reader structure"]
impl crate::Readable for WDTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcsr::W](W) writer structure"]
impl crate::Writable for WDTCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCSR to value 0"]
impl crate::Resettable for WDTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
