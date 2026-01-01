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
    #[doc = "0: Prescaler Value 1"]
    PRESCALER_1 = 0,
    #[doc = "1: Prescaler Value 2"]
    PRESCALER_2 = 1,
    #[doc = "2: Prescaler Value 4"]
    PRESCALER_4 = 2,
    #[doc = "3: Prescaler Value 8"]
    PRESCALER_8 = 3,
    #[doc = "4: Prescaler Value 16"]
    PRESCALER_16 = 4,
    #[doc = "5: Prescaler Value 32"]
    PRESCALER_32 = 5,
    #[doc = "6: Prescaler Value 64"]
    PRESCALER_64 = 6,
    #[doc = "7: Prescaler Value 128"]
    PRESCALER_128 = 7,
    #[doc = "8: Prescaler Value 256"]
    PRESCALER_256 = 8,
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
            0 => Some(CLKPS_A::PRESCALER_1),
            1 => Some(CLKPS_A::PRESCALER_2),
            2 => Some(CLKPS_A::PRESCALER_4),
            3 => Some(CLKPS_A::PRESCALER_8),
            4 => Some(CLKPS_A::PRESCALER_16),
            5 => Some(CLKPS_A::PRESCALER_32),
            6 => Some(CLKPS_A::PRESCALER_64),
            7 => Some(CLKPS_A::PRESCALER_128),
            8 => Some(CLKPS_A::PRESCALER_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline(always)]
    pub fn is_prescaler_1(&self) -> bool {
        *self == CLKPS_A::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_2`"]
    #[inline(always)]
    pub fn is_prescaler_2(&self) -> bool {
        *self == CLKPS_A::PRESCALER_2
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4`"]
    #[inline(always)]
    pub fn is_prescaler_4(&self) -> bool {
        *self == CLKPS_A::PRESCALER_4
    }
    #[doc = "Checks if the value of the field is `PRESCALER_8`"]
    #[inline(always)]
    pub fn is_prescaler_8(&self) -> bool {
        *self == CLKPS_A::PRESCALER_8
    }
    #[doc = "Checks if the value of the field is `PRESCALER_16`"]
    #[inline(always)]
    pub fn is_prescaler_16(&self) -> bool {
        *self == CLKPS_A::PRESCALER_16
    }
    #[doc = "Checks if the value of the field is `PRESCALER_32`"]
    #[inline(always)]
    pub fn is_prescaler_32(&self) -> bool {
        *self == CLKPS_A::PRESCALER_32
    }
    #[doc = "Checks if the value of the field is `PRESCALER_64`"]
    #[inline(always)]
    pub fn is_prescaler_64(&self) -> bool {
        *self == CLKPS_A::PRESCALER_64
    }
    #[doc = "Checks if the value of the field is `PRESCALER_128`"]
    #[inline(always)]
    pub fn is_prescaler_128(&self) -> bool {
        *self == CLKPS_A::PRESCALER_128
    }
    #[doc = "Checks if the value of the field is `PRESCALER_256`"]
    #[inline(always)]
    pub fn is_prescaler_256(&self) -> bool {
        *self == CLKPS_A::PRESCALER_256
    }
}
#[doc = "Field `CLKPS` writer - Clock Prescaler Select Bits"]
pub type CLKPS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLKPR_SPEC, u8, CLKPS_A, 4, O>;
impl<'a, const O: u8> CLKPS_W<'a, O> {
    #[doc = "Prescaler Value 1"]
    #[inline(always)]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_1)
    }
    #[doc = "Prescaler Value 2"]
    #[inline(always)]
    pub fn prescaler_2(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_2)
    }
    #[doc = "Prescaler Value 4"]
    #[inline(always)]
    pub fn prescaler_4(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_4)
    }
    #[doc = "Prescaler Value 8"]
    #[inline(always)]
    pub fn prescaler_8(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_8)
    }
    #[doc = "Prescaler Value 16"]
    #[inline(always)]
    pub fn prescaler_16(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_16)
    }
    #[doc = "Prescaler Value 32"]
    #[inline(always)]
    pub fn prescaler_32(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_32)
    }
    #[doc = "Prescaler Value 64"]
    #[inline(always)]
    pub fn prescaler_64(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_64)
    }
    #[doc = "Prescaler Value 128"]
    #[inline(always)]
    pub fn prescaler_128(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_128)
    }
    #[doc = "Prescaler Value 256"]
    #[inline(always)]
    pub fn prescaler_256(self) -> &'a mut W {
        self.variant(CLKPS_A::PRESCALER_256)
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
