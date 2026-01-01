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
#[doc = "Field `WDP` reader - Watchdog Timer Prescaler Bits"]
pub type WDP_R = crate::FieldReader<u8, WDP_A>;
#[doc = "Watchdog Timer Prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDP_A {
    #[doc = "0: Oscillator Cycles 2K"]
    VAL_0X00 = 0,
    #[doc = "1: Oscillator Cycles 4K"]
    VAL_0X01 = 1,
    #[doc = "2: Oscillator Cycles 8K"]
    VAL_0X02 = 2,
    #[doc = "3: Oscillator Cycles 16K"]
    VAL_0X03 = 3,
    #[doc = "4: Oscillator Cycles 32K"]
    VAL_0X04 = 4,
    #[doc = "5: Oscillator Cycles 64K"]
    VAL_0X05 = 5,
    #[doc = "6: Oscillator Cycles 128K"]
    VAL_0X06 = 6,
    #[doc = "7: Oscillator Cycles 256K"]
    VAL_0X07 = 7,
}
impl From<WDP_A> for u8 {
    #[inline(always)]
    fn from(variant: WDP_A) -> Self {
        variant as _
    }
}
impl WDP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDP_A> {
        match self.bits {
            0 => Some(WDP_A::VAL_0X00),
            1 => Some(WDP_A::VAL_0X01),
            2 => Some(WDP_A::VAL_0X02),
            3 => Some(WDP_A::VAL_0X03),
            4 => Some(WDP_A::VAL_0X04),
            5 => Some(WDP_A::VAL_0X05),
            6 => Some(WDP_A::VAL_0X06),
            7 => Some(WDP_A::VAL_0X07),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == WDP_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == WDP_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == WDP_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == WDP_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == WDP_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == WDP_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == WDP_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == WDP_A::VAL_0X07
    }
}
#[doc = "Field `WDP` writer - Watchdog Timer Prescaler Bits"]
pub type WDP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, WDTCSR_SPEC, u8, WDP_A, 6, O>;
impl<'a, const O: u8> WDP_W<'a, O> {
    #[doc = "Oscillator Cycles 2K"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X00)
    }
    #[doc = "Oscillator Cycles 4K"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X01)
    }
    #[doc = "Oscillator Cycles 8K"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X02)
    }
    #[doc = "Oscillator Cycles 16K"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X03)
    }
    #[doc = "Oscillator Cycles 32K"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X04)
    }
    #[doc = "Oscillator Cycles 64K"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X05)
    }
    #[doc = "Oscillator Cycles 128K"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X06)
    }
    #[doc = "Oscillator Cycles 256K"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(WDP_A::VAL_0X07)
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
#[doc = "Field `WDIE` reader - Watchdog Timeout Interrupt Enable"]
pub type WDIE_R = crate::BitReader<bool>;
#[doc = "Field `WDIE` writer - Watchdog Timeout Interrupt Enable"]
pub type WDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
#[doc = "Field `WDIF` reader - Watchdog Timeout Interrupt Flag"]
pub type WDIF_R = crate::BitReader<bool>;
#[doc = "Field `WDIF` writer - Watchdog Timeout Interrupt Flag"]
pub type WDIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, WDTCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Watchdog Timer Prescaler Bits"]
    #[inline(always)]
    pub fn wdp(&self) -> WDP_R {
        WDP_R::new(self.bits & 0x3f)
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
    #[doc = "Bits 0:5 - Watchdog Timer Prescaler Bits"]
    #[inline(always)]
    #[must_use]
    pub fn wdp(&mut self) -> WDP_W<0> {
        WDP_W::new(self)
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
