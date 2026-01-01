#[doc = "Register `ADCSRB` reader"]
pub struct R(crate::R<ADCSRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSRB` writer"]
pub struct W(crate::W<ADCSRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSRB_SPEC>;
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
impl From<crate::W<ADCSRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADTS` reader - ADC Auto Trigger Source bits"]
pub type ADTS_R = crate::FieldReader<u8, ADTS_A>;
#[doc = "ADC Auto Trigger Source bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTS_A {
    #[doc = "0: Free Running mode"]
    VAL_0X00 = 0,
    #[doc = "1: Analog Comparator"]
    VAL_0X01 = 1,
    #[doc = "2: External Interrupt Request 0"]
    VAL_0X02 = 2,
    #[doc = "3: Timer/Counter0 Compare Match A"]
    VAL_0X03 = 3,
    #[doc = "4: Timer/Counter0 Overflow"]
    VAL_0X04 = 4,
    #[doc = "5: Timer/Counter1 Compare Match B"]
    VAL_0X05 = 5,
    #[doc = "6: Timer/Counter1 Overflow"]
    VAL_0X06 = 6,
    #[doc = "7: Timer/Counter1 Capture Event"]
    VAL_0X07 = 7,
}
impl From<ADTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTS_A) -> Self {
        variant as _
    }
}
impl ADTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTS_A {
        match self.bits {
            0 => ADTS_A::VAL_0X00,
            1 => ADTS_A::VAL_0X01,
            2 => ADTS_A::VAL_0X02,
            3 => ADTS_A::VAL_0X03,
            4 => ADTS_A::VAL_0X04,
            5 => ADTS_A::VAL_0X05,
            6 => ADTS_A::VAL_0X06,
            7 => ADTS_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ADTS_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ADTS_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ADTS_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ADTS_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == ADTS_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == ADTS_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == ADTS_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == ADTS_A::VAL_0X07
    }
}
#[doc = "Field `ADTS` writer - ADC Auto Trigger Source bits"]
pub type ADTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADCSRB_SPEC, u8, ADTS_A, 3, O>;
impl<'a, const O: u8> ADTS_W<'a, O> {
    #[doc = "Free Running mode"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X00)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X01)
    }
    #[doc = "External Interrupt Request 0"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X02)
    }
    #[doc = "Timer/Counter0 Compare Match A"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X03)
    }
    #[doc = "Timer/Counter0 Overflow"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X04)
    }
    #[doc = "Timer/Counter1 Compare Match B"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X05)
    }
    #[doc = "Timer/Counter1 Overflow"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X06)
    }
    #[doc = "Timer/Counter1 Capture Event"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(ADTS_A::VAL_0X07)
    }
}
#[doc = "Field `MUX5` reader - Analog Channel and Gain Selection Bits"]
pub type MUX5_R = crate::BitReader<bool>;
#[doc = "Field `MUX5` writer - Analog Channel and Gain Selection Bits"]
pub type MUX5_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCSRB_SPEC, bool, O>;
#[doc = "Field `ACME` reader - No Description."]
pub type ACME_R = crate::BitReader<bool>;
#[doc = "Field `ACME` writer - No Description."]
pub type ACME_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCSRB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - ADC Auto Trigger Source bits"]
    #[inline(always)]
    pub fn adts(&self) -> ADTS_R {
        ADTS_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn acme(&self) -> ACME_R {
        ACME_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Auto Trigger Source bits"]
    #[inline(always)]
    #[must_use]
    pub fn adts(&mut self) -> ADTS_W<0> {
        ADTS_W::new(self)
    }
    #[doc = "Bit 3 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux5(&mut self) -> MUX5_W<3> {
        MUX5_W::new(self)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn acme(&mut self) -> ACME_W<6> {
        ACME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Control and Status register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcsrb](index.html) module"]
pub struct ADCSRB_SPEC;
impl crate::RegisterSpec for ADCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcsrb::R](R) reader structure"]
impl crate::Readable for ADCSRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcsrb::W](W) writer structure"]
impl crate::Writable for ADCSRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRB to value 0"]
impl crate::Resettable for ADCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
