#[doc = "Register `CLKPR` reader"]
pub struct R(crate::R<CLKPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKPR` writer"]
pub struct W(crate::W<CLKPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPR_SPEC>;
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
impl From<crate::W<CLKPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPS` reader - Clock Prescaler Select Bits"]
pub type CLKPS_R = crate::FieldReader<u8, CLKPS_A>;
#[doc = "Clock Prescaler Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKPS_A {
    #[doc = "0: 1"]
    VAL_0X00 = 0,
    #[doc = "1: 2"]
    VAL_0X01 = 1,
    #[doc = "2: 4"]
    VAL_0X02 = 2,
    #[doc = "3: 8"]
    VAL_0X03 = 3,
    #[doc = "4: 16"]
    VAL_0X04 = 4,
    #[doc = "5: 32"]
    VAL_0X05 = 5,
    #[doc = "6: 64"]
    VAL_0X06 = 6,
    #[doc = "7: 128"]
    VAL_0X07 = 7,
    #[doc = "8: 256"]
    VAL_0X08 = 8,
}
impl From<CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKPS_A) -> Self {
        variant as _
    }
}
impl CLKPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKPS_A> {
        match self.bits {
            0 => Some(CLKPS_A::VAL_0X00),
            1 => Some(CLKPS_A::VAL_0X01),
            2 => Some(CLKPS_A::VAL_0X02),
            3 => Some(CLKPS_A::VAL_0X03),
            4 => Some(CLKPS_A::VAL_0X04),
            5 => Some(CLKPS_A::VAL_0X05),
            6 => Some(CLKPS_A::VAL_0X06),
            7 => Some(CLKPS_A::VAL_0X07),
            8 => Some(CLKPS_A::VAL_0X08),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CLKPS_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CLKPS_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CLKPS_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CLKPS_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CLKPS_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CLKPS_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CLKPS_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CLKPS_A::VAL_0X07
    }
    #[doc = "Checks if the value of the field is `VAL_0X08`"]
    #[inline(always)]
    pub fn is_val_0x08(&self) -> bool {
        *self == CLKPS_A::VAL_0X08
    }
}
#[doc = "Field `CLKPS` writer - Clock Prescaler Select Bits"]
pub type CLKPS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLKPR_SPEC, u8, CLKPS_A, 4, O>;
impl<'a, const O: u8> CLKPS_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X00)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X01)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X02)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X03)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X04)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X05)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X06)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X07)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn val_0x08(self) -> &'a mut W {
        self.variant(CLKPS_A::VAL_0X08)
    }
}
#[doc = "Field `CLKPCE` reader - Clock Prescaler Change Enable"]
pub type CLKPCE_R = crate::BitReader<bool>;
#[doc = "Field `CLKPCE` writer - Clock Prescaler Change Enable"]
pub type CLKPCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKPR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Prescaler Select Bits"]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Clock Prescaler Change Enable"]
    #[inline(always)]
    pub fn clkpce(&self) -> CLKPCE_R {
        CLKPCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Prescaler Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<0> {
        CLKPS_W::new(self)
    }
    #[doc = "Bit 7 - Clock Prescaler Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkpce(&mut self) -> CLKPCE_W<7> {
        CLKPCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Prescale Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpr](index.html) module"]
pub struct CLKPR_SPEC;
impl crate::RegisterSpec for CLKPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkpr::R](R) reader structure"]
impl crate::Readable for CLKPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpr::W](W) writer structure"]
impl crate::Writable for CLKPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKPR to value 0"]
impl crate::Resettable for CLKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
