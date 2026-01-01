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
    #[doc = "0: Division factor 1 / RC-Oscillator 2"]
    DIVISION_FACTOR_1_RC_OSCILLATOR_2 = 0,
    #[doc = "1: Division factor 2 / RC-Oscillator 4"]
    DIVISION_FACTOR_2_RC_OSCILLATOR_4 = 1,
    #[doc = "2: Division factor 4 / RC-Oscillator 8"]
    DIVISION_FACTOR_4_RC_OSCILLATOR_8 = 2,
    #[doc = "3: Division factor 8 / RC-Oscillator 16"]
    DIVISION_FACTOR_8_RC_OSCILLATOR_16 = 3,
    #[doc = "4: Division factor 16 / RC-Oscillator 32"]
    DIVISION_FACTOR_16_RC_OSCILLATOR_32 = 4,
    #[doc = "5: Division factor 32 / RC-Oscillator 64"]
    DIVISION_FACTOR_32_RC_OSCILLATOR_64 = 5,
    #[doc = "6: Division factor 64 / RC-Oscillator 128"]
    DIVISION_FACTOR_64_RC_OSCILLATOR_128 = 6,
    #[doc = "7: Division factor 128 / RC-Oscillator 256"]
    DIVISION_FACTOR_128_RC_OSCILLATOR_256 = 7,
    #[doc = "8: Division factor 256 / RC-Oscillator 512"]
    DIVISION_FACTOR_256_RC_OSCILLATOR_512 = 8,
    #[doc = "15: Division factor 1 only permitted for RC-Oscillator. Flash and EEPROM programming is not allowed."]
    DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED =
        15,
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
        match self . bits { 0 => Some ( CLKPS_A :: DIVISION_FACTOR_1_RC_OSCILLATOR_2 ) , 1 => Some ( CLKPS_A :: DIVISION_FACTOR_2_RC_OSCILLATOR_4 ) , 2 => Some ( CLKPS_A :: DIVISION_FACTOR_4_RC_OSCILLATOR_8 ) , 3 => Some ( CLKPS_A :: DIVISION_FACTOR_8_RC_OSCILLATOR_16 ) , 4 => Some ( CLKPS_A :: DIVISION_FACTOR_16_RC_OSCILLATOR_32 ) , 5 => Some ( CLKPS_A :: DIVISION_FACTOR_32_RC_OSCILLATOR_64 ) , 6 => Some ( CLKPS_A :: DIVISION_FACTOR_64_RC_OSCILLATOR_128 ) , 7 => Some ( CLKPS_A :: DIVISION_FACTOR_128_RC_OSCILLATOR_256 ) , 8 => Some ( CLKPS_A :: DIVISION_FACTOR_256_RC_OSCILLATOR_512 ) , 15 => Some ( CLKPS_A :: DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED ) , _ => None , }
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_1_RC_OSCILLATOR_2`"]
    #[inline(always)]
    pub fn is_division_factor_1_rc_oscillator_2(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_1_RC_OSCILLATOR_2
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_2_RC_OSCILLATOR_4`"]
    #[inline(always)]
    pub fn is_division_factor_2_rc_oscillator_4(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_2_RC_OSCILLATOR_4
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_4_RC_OSCILLATOR_8`"]
    #[inline(always)]
    pub fn is_division_factor_4_rc_oscillator_8(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_4_RC_OSCILLATOR_8
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_8_RC_OSCILLATOR_16`"]
    #[inline(always)]
    pub fn is_division_factor_8_rc_oscillator_16(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_8_RC_OSCILLATOR_16
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_16_RC_OSCILLATOR_32`"]
    #[inline(always)]
    pub fn is_division_factor_16_rc_oscillator_32(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_16_RC_OSCILLATOR_32
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_32_RC_OSCILLATOR_64`"]
    #[inline(always)]
    pub fn is_division_factor_32_rc_oscillator_64(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_32_RC_OSCILLATOR_64
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_64_RC_OSCILLATOR_128`"]
    #[inline(always)]
    pub fn is_division_factor_64_rc_oscillator_128(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_64_RC_OSCILLATOR_128
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_128_RC_OSCILLATOR_256`"]
    #[inline(always)]
    pub fn is_division_factor_128_rc_oscillator_256(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_128_RC_OSCILLATOR_256
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_256_RC_OSCILLATOR_512`"]
    #[inline(always)]
    pub fn is_division_factor_256_rc_oscillator_512(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_256_RC_OSCILLATOR_512
    }
    #[doc = "Checks if the value of the field is `DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_division_factor_1_only_permitted_for_rc_oscillator_flash_and_eeprom_programming_is_not_allowed(
        &self,
    ) -> bool {
        * self == CLKPS_A :: DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED
    }
}
#[doc = "Field `CLKPS` writer - Clock Prescaler Select Bits"]
pub type CLKPS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLKPR_SPEC, u8, CLKPS_A, 4, O>;
impl<'a, const O: u8> CLKPS_W<'a, O> {
    #[doc = "Division factor 1 / RC-Oscillator 2"]
    #[inline(always)]
    pub fn division_factor_1_rc_oscillator_2(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_1_RC_OSCILLATOR_2)
    }
    #[doc = "Division factor 2 / RC-Oscillator 4"]
    #[inline(always)]
    pub fn division_factor_2_rc_oscillator_4(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_2_RC_OSCILLATOR_4)
    }
    #[doc = "Division factor 4 / RC-Oscillator 8"]
    #[inline(always)]
    pub fn division_factor_4_rc_oscillator_8(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_4_RC_OSCILLATOR_8)
    }
    #[doc = "Division factor 8 / RC-Oscillator 16"]
    #[inline(always)]
    pub fn division_factor_8_rc_oscillator_16(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_8_RC_OSCILLATOR_16)
    }
    #[doc = "Division factor 16 / RC-Oscillator 32"]
    #[inline(always)]
    pub fn division_factor_16_rc_oscillator_32(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_16_RC_OSCILLATOR_32)
    }
    #[doc = "Division factor 32 / RC-Oscillator 64"]
    #[inline(always)]
    pub fn division_factor_32_rc_oscillator_64(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_32_RC_OSCILLATOR_64)
    }
    #[doc = "Division factor 64 / RC-Oscillator 128"]
    #[inline(always)]
    pub fn division_factor_64_rc_oscillator_128(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_64_RC_OSCILLATOR_128)
    }
    #[doc = "Division factor 128 / RC-Oscillator 256"]
    #[inline(always)]
    pub fn division_factor_128_rc_oscillator_256(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_128_RC_OSCILLATOR_256)
    }
    #[doc = "Division factor 256 / RC-Oscillator 512"]
    #[inline(always)]
    pub fn division_factor_256_rc_oscillator_512(self) -> &'a mut W {
        self.variant(CLKPS_A::DIVISION_FACTOR_256_RC_OSCILLATOR_512)
    }
    #[doc = "Division factor 1 only permitted for RC-Oscillator. Flash and EEPROM programming is not allowed."]
    #[inline(always)]
    pub fn division_factor_1_only_permitted_for_rc_oscillator_flash_and_eeprom_programming_is_not_allowed(
        self,
    ) -> &'a mut W {
        self . variant ( CLKPS_A :: DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED )
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKPR_SPEC, u8, u8, 3, O>;
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
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 7)
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
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<4> {
        RES_W::new(self)
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
