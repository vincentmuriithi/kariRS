#[doc = "Register `CLKCSR` reader"]
pub struct R(crate::R<CLKCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCSR` writer"]
pub struct W(crate::W<CLKCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCSR_SPEC>;
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
impl From<crate::W<CLKCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKC` reader - Clock Control bits"]
pub type CLKC_R = crate::FieldReader<u8, CLKC_A>;
#[doc = "Clock Control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKC_A {
    #[doc = "0: No Command"]
    VAL_0X00 = 0,
    #[doc = "1: Disable Clock Source"]
    VAL_0X01 = 1,
    #[doc = "2: Enable Clock Source"]
    VAL_0X02 = 2,
    #[doc = "3: Request for Clock Availability"]
    VAL_0X03 = 3,
    #[doc = "4: Clock Source Switch"]
    VAL_0X04 = 4,
    #[doc = "5: Recovery System Clock Source Code"]
    VAL_0X05 = 5,
    #[doc = "6: Enable Watchdog in Automatic Reload Mode"]
    VAL_0X06 = 6,
    #[doc = "7: CKOUT Command"]
    VAL_0X07 = 7,
    #[doc = "8: From 0x08 up to 0x0F: No command"]
    VAL_0X08 = 8,
}
impl From<CLKC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKC_A) -> Self {
        variant as _
    }
}
impl CLKC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKC_A> {
        match self.bits {
            0 => Some(CLKC_A::VAL_0X00),
            1 => Some(CLKC_A::VAL_0X01),
            2 => Some(CLKC_A::VAL_0X02),
            3 => Some(CLKC_A::VAL_0X03),
            4 => Some(CLKC_A::VAL_0X04),
            5 => Some(CLKC_A::VAL_0X05),
            6 => Some(CLKC_A::VAL_0X06),
            7 => Some(CLKC_A::VAL_0X07),
            8 => Some(CLKC_A::VAL_0X08),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CLKC_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CLKC_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CLKC_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CLKC_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CLKC_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CLKC_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CLKC_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CLKC_A::VAL_0X07
    }
    #[doc = "Checks if the value of the field is `VAL_0X08`"]
    #[inline(always)]
    pub fn is_val_0x08(&self) -> bool {
        *self == CLKC_A::VAL_0X08
    }
}
#[doc = "Field `CLKC` writer - Clock Control bits"]
pub type CLKC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLKCSR_SPEC, u8, CLKC_A, 4, O>;
impl<'a, const O: u8> CLKC_W<'a, O> {
    #[doc = "No Command"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X00)
    }
    #[doc = "Disable Clock Source"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X01)
    }
    #[doc = "Enable Clock Source"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X02)
    }
    #[doc = "Request for Clock Availability"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X03)
    }
    #[doc = "Clock Source Switch"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X04)
    }
    #[doc = "Recovery System Clock Source Code"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X05)
    }
    #[doc = "Enable Watchdog in Automatic Reload Mode"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X06)
    }
    #[doc = "CKOUT Command"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X07)
    }
    #[doc = "From 0x08 up to 0x0F: No command"]
    #[inline(always)]
    pub fn val_0x08(self) -> &'a mut W {
        self.variant(CLKC_A::VAL_0X08)
    }
}
#[doc = "Field `CLKRDY` reader - Clock Ready Flag"]
pub type CLKRDY_R = crate::BitReader<bool>;
#[doc = "Field `CLKRDY` writer - Clock Ready Flag"]
pub type CLKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKCSR_SPEC, bool, O>;
#[doc = "Field `CLKCCE` reader - Clock Control Change Enable"]
pub type CLKCCE_R = crate::BitReader<bool>;
#[doc = "Field `CLKCCE` writer - Clock Control Change Enable"]
pub type CLKCCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Control bits"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Clock Ready Flag"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Control Change Enable"]
    #[inline(always)]
    pub fn clkcce(&self) -> CLKCCE_R {
        CLKCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Control bits"]
    #[inline(always)]
    #[must_use]
    pub fn clkc(&mut self) -> CLKC_W<0> {
        CLKC_W::new(self)
    }
    #[doc = "Bit 4 - Clock Ready Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy(&mut self) -> CLKRDY_W<4> {
        CLKRDY_W::new(self)
    }
    #[doc = "Bit 7 - Clock Control Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkcce(&mut self) -> CLKCCE_W<7> {
        CLKCCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control & Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcsr](index.html) module"]
pub struct CLKCSR_SPEC;
impl crate::RegisterSpec for CLKCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkcsr::R](R) reader structure"]
impl crate::Readable for CLKCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcsr::W](W) writer structure"]
impl crate::Writable for CLKCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCSR to value 0"]
impl crate::Resettable for CLKCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
